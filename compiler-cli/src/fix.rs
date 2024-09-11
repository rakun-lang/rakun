use camino::{Utf8Path, Utf8PathBuf};
use rakun_core::{
    error::{FileIoAction, FileKind},
    Error, Result,
};

pub fn run() -> Result<()> {
    for path in crate::fs::rakun_files_excluding_gitignore(Utf8Path::new(".")) {
        fix_file(path)?;
    }

    // Set the version requirement in rakun.toml
    let mut toml = crate::fs::read("rakun.toml")?
        .parse::<toml_edit::DocumentMut>()
        .map_err(|e| Error::FileIo {
            kind: FileKind::File,
            action: FileIoAction::Parse,
            path: Utf8PathBuf::from("rakun.toml"),
            err: Some(e.to_string()),
        })?;

    #[allow(clippy::indexing_slicing)]
    {
        toml["rakun"] = toml_edit::value(">= 0.32.0");
    }

    // Write the updated config
    crate::fs::write(Utf8Path::new("rakun.toml"), &toml.to_string())?;

    println!(
        "Your Rakun code has been fixed!

If you have any JavaScript code that used the BitString class
you will need to update it to use the BitArray class instead.
"
    );
    Ok(())
}

fn fix_file(path: Utf8PathBuf) -> Result<()> {
    let src = crate::fs::read(&path)?;
    let out = rakun_core::fix::parse_fix_and_format(&src.into(), &path)?;
    crate::fs::write(&path, &out)?;
    Ok(())
}
