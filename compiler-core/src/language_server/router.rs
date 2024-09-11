use crate::{
    build::SourceFingerprint,
    error::{FileIoAction, FileKind},
    io::{CommandExecutor, FileSystemReader, FileSystemWriter},
    language_server::{
        engine::LanguageServerEngine, files::FileSystemProxy, progress::ProgressReporter,
        DownloadDependencies, MakeLocker,
    },
    paths::ProjectPaths,
    Error, Result,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    time::SystemTime,
};

use camino::{Utf8Path, Utf8PathBuf};

use super::feedback::FeedbackBookKeeper;

/// The language server instance serves a language client, typically a text
/// editor. The editor could have multiple Rakun projects open at once, so run
/// an instance of the language server engine for each project.
///
/// This router is responsible for finding or creating an engine for a given
/// file using the nearest parent `rakun.toml` file.
///
#[derive(Debug)]
pub(crate) struct Router<IO, Reporter> {
    io: FileSystemProxy<IO>,
    engines: HashMap<Utf8PathBuf, Project<IO, Reporter>>,
    progress_reporter: Reporter,
}

impl<'a, IO, Reporter> Router<IO, Reporter>
where
    // IO to be supplied from outside of rakun-core
    IO: FileSystemReader
        + FileSystemWriter
        + CommandExecutor
        + DownloadDependencies
        + MakeLocker
        + Clone,
    // IO to be supplied from inside of rakun-core
    Reporter: ProgressReporter + Clone + 'a,
{
    pub fn new(progress_reporter: Reporter, io: FileSystemProxy<IO>) -> Self {
        Self {
            io,
            engines: HashMap::new(),
            progress_reporter,
        }
    }

    pub fn project_path(&self, path: &Utf8Path) -> Option<Utf8PathBuf> {
        find_rakun_project_parent(&self.io, path)
    }

    pub fn project_for_path(
        &mut self,
        path: Utf8PathBuf,
    ) -> Result<Option<&mut Project<IO, Reporter>>> {
        // If the path is the root of a known project then return it. Otherwise
        // find the nearest parent project.
        let path = if self.engines.contains_key(&path) {
            path
        } else {
            let Some(path) = find_rakun_project_parent(&self.io, &path) else {
                return Ok(None);
            };
            path
        };

        // If the rakun.toml has changed then discard the project as the target,
        // deps, etc may have changed and we need to rebuild taking them into
        // account.
        if let Some(project) = self.engines.get(&path) {
            if Self::rakun_toml_changed(&path, project, &self.io)? {
                let _ = self.engines.remove(&path);
            }
        }

        // Look up the project, creating a new one if it does not exist.
        Ok(Some(match self.engines.entry(path.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let project =
                    Self::new_project(path, self.io.clone(), self.progress_reporter.clone())?;
                entry.insert(project)
            }
        }))
    }

    /// Has rakun.toml changed since the last time we saw this project?
    fn rakun_toml_changed(
        path: &Utf8PathBuf,
        project: &Project<IO, Reporter>,
        io: &FileSystemProxy<IO>,
    ) -> Result<bool, Error> {
        // Get the location of rakun.toml for this project
        let paths = ProjectPaths::new(path.clone());
        let config_path = paths.root_config();

        // See if the file modification time has changed.
        if io.modification_time(path)? == project.rakun_toml_modification_time {
            return Ok(false); // Not changed
        }

        // The mtime has changed. This might not be a content change, so let's
        // check the hash.
        let toml = io.read(&config_path)?;
        let rakun_toml_changed = project.rakun_toml_fingerprint != SourceFingerprint::new(&toml);
        Ok(rakun_toml_changed)
    }

    pub fn delete_engine_for_path(&mut self, path: &Utf8Path) {
        if let Some(path) = find_rakun_project_parent(&self.io, path) {
            _ = self.engines.remove(&path);
        }
    }

    fn new_project(
        path: Utf8PathBuf,
        io: FileSystemProxy<IO>,
        progress_reporter: Reporter,
    ) -> Result<Project<IO, Reporter>, Error> {
        tracing::info!(?path, "creating_new_language_server_engine");
        let paths = ProjectPaths::new(path);
        let config_path = paths.root_config();
        let modification_time = io.modification_time(&config_path)?;
        let toml = io.read(&config_path)?;
        let config = toml::from_str(&toml).map_err(|e| Error::FileIo {
            action: FileIoAction::Parse,
            kind: FileKind::File,
            path: config_path,
            err: Some(e.to_string()),
        })?;
        let engine = LanguageServerEngine::new(config, progress_reporter, io, paths)?;
        let project = Project {
            engine,
            feedback: FeedbackBookKeeper::default(),
            rakun_toml_modification_time: modification_time,
            rakun_toml_fingerprint: SourceFingerprint::new(&toml),
        };
        Ok(project)
    }
}

/// Given a given path, find the nearest parent directory containing a
/// `rakun.toml` file.
///
/// The file must be in either the `src` or `test` directory if it is not a
/// `.rakun` file.
fn find_rakun_project_parent<IO>(io: &IO, path: &Utf8Path) -> Option<Utf8PathBuf>
where
    IO: FileSystemReader,
{
    let is_module = path.extension().map(|x| x == "rakun").unwrap_or(false);
    let mut directory = path.to_path_buf();

    // If we are finding the rakun project of a directory then we want to check the directory itself
    let is_directory = path.extension().is_none();
    if is_directory {
        directory.push("src");
    }

    while let Some(root) = directory.parent() {
        // If there's no rakun.toml in the root then we continue to the next parent.
        if !io.is_file(&root.join("rakun.toml")) {
            _ = directory.pop();
            continue;
        }

        // If it is a Rakun module then it must reside in the src or test directory.
        if is_module && !(directory.ends_with("test") || directory.ends_with("src")) {
            _ = directory.pop();
            continue;
        }

        return Some(root.to_path_buf());
    }
    None
}

#[derive(Debug)]
pub(crate) struct Project<A, B> {
    pub engine: LanguageServerEngine<A, B>,
    pub feedback: FeedbackBookKeeper,
    pub rakun_toml_modification_time: SystemTime,
    pub rakun_toml_fingerprint: SourceFingerprint,
}

#[cfg(test)]
mod find_rakun_project_parent_tests {
    use super::*;
    use crate::io::{memory::InMemoryFileSystem, FileSystemWriter};

    #[test]
    fn root() {
        let io = InMemoryFileSystem::new();
        assert_eq!(find_rakun_project_parent(&io, Utf8Path::new("/")), None);
    }

    #[test]
    fn outside_a_project() {
        let io = InMemoryFileSystem::new();
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app/src/one.rakun")),
            None
        );
    }

    #[test]
    fn rakun_toml_itself() {
        let io = InMemoryFileSystem::new();
        io.write(Utf8Path::new("/app/rakun.toml"), "").unwrap();
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app/rakun.toml")),
            Some(Utf8PathBuf::from("/app"))
        );
    }

    #[test]
    fn directory_with_rakun_toml() {
        let io = InMemoryFileSystem::new();
        io.write(Utf8Path::new("/app/rakun.toml"), "").unwrap();
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app")),
            Some(Utf8PathBuf::from("/app"))
        );
    }

    #[test]
    fn test_module() {
        let io = InMemoryFileSystem::new();
        io.write(Utf8Path::new("/app/rakun.toml"), "").unwrap();
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app/test/one/two/three.rakun")),
            Some(Utf8PathBuf::from("/app"))
        );
    }

    #[test]
    fn src_module() {
        let io = InMemoryFileSystem::new();
        io.write(Utf8Path::new("/app/rakun.toml"), "").unwrap();
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app/src/one/two/three.rakun")),
            Some(Utf8PathBuf::from("/app"))
        );
    }

    // https://github.com/rakun-lang/rakun/issues/2121
    #[test]
    fn module_in_project_but_not_src_or_test() {
        let io = InMemoryFileSystem::new();
        io.write(Utf8Path::new("/app/rakun.toml"), "").unwrap();
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app/other/one/two/three.rakun")),
            None,
        );
    }

    #[test]
    fn nested_projects() {
        let io = InMemoryFileSystem::new();
        io.write(Utf8Path::new("/app/rakun.toml"), "").unwrap();
        io.write(Utf8Path::new("/app/examples/wibble/rakun.toml"), "")
            .unwrap();
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app/src/one.rakun")),
            Some(Utf8PathBuf::from("/app"))
        );
        assert_eq!(
            find_rakun_project_parent(&io, Utf8Path::new("/app/examples/wibble/src/one.rakun")),
            Some(Utf8PathBuf::from("/app/examples/wibble"))
        );
    }
}
