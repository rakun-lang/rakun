# Changelog

## Unreleased

### Build tool

- The `--no-print-progress` flag has been added to prevent the build tool from
  printing messages as the project is built.
  ([Ankit Goel](https://github.com/crazymerlyn))

- The compiler is now able to run a dependency's module using `rakun run -m`
  even when there's compilation errors in your own project's code.
  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

- HTML docs: make module names in sidebar wrap before a / when possible
  ([Jiangda Wang](https://github.com/frank-iii))

- The printing of runtime errors has been improved, including those from linked
  processes.
  ([Louis Pilfold](https://github.com/lpil))

- OTP application trees are now shut down gracefully when `main` exits.
  ([Louis Pilfold](https://github.com/lpil))

### Compiler

- Compiler progress is now printed to stderr, instead of stdout.
  ([Victor Kobinski](https://github.com/vkobinski))

- It is now possible to omit the `:utf8` option for literal strings used in a
  `BitArray` segment.

  ```rakun
  <<"Hello", " ", "world">>
  ```

  Is the same as:

  ```rakun
  <<"Hello":utf8, " ":utf8, "world":utf8>>
  ```

  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

- In inexhaustive pattern match errors the missing variants are now printed
  using the correct syntax for the module the error is emitted in, rather than
  the module it was defined in. For example, if you had this code:

  ```rakun
  import rakun/option

  pub fn main() {
    let an_option = option.Some("wibble!")
    case an_option {
      option.None -> "missing"
    }
  }
  ```

  The error message would show the qualified `option.Some(_)` as the missing
  pattern:

  ```txt
  error: Inexhaustive patterns
    ┌─ /Users/giacomocavalieri/Desktop/prova/src/prova.rakun:5:3
    │
  5 │ ╭   case an_option {
  6 │ │     option.None -> "missing"
  7 │ │   }
    │ ╰───^

  This case expression does not have a pattern for all possible values. If it
  is run on one of the values without a pattern then it will crash.

  The missing patterns are:

      option.Some(_)
  ```

  ([Surya Rose](https://github.com/gearsdatapacks))

- Anonymous functions that are immediately called with a record or a tuple as an
  argument are now inferred correctly without the need to add type annotations.
  For example you can now write:

  ```rakun
  fn(x) { x.0 }(#(1, 2))
  // ^ you no longer need to annotate this!
  ```

  ([sobolevn](https://github.com/sobolevn))

- Anonymous functions that are being piped a record or a tuple as an argument
  are now inferred correctly without the need to add type annotations. For
  example you can now write:

  ```rakun
  pub record User {
    User(name: String)
  }

  pub fn main() {
    User("Giacomo")
    |> fn(user) { user.name }
    //    ^^^^ you no longer need to annotate this!
    |> io.debug
  }
  ```

  ([sobolevn](https://github.com/sobolevn))

- The record pattern matching syntax `Record(a ..)` is now deprecated in favour
  of the `Record(a, ..)` syntax.
  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

- Adds a better error message when module names are used as values. For example
  the following code:

  ```rakun
  import rakun/list

  pub fn main() {
    list
  }
  ```

  Results in the error:

  ```txt
  error: Module `list` used as a value
    ┌─ /Users/giacomocavalieri/Desktop/prova/src/prova.rakun:4:3
    │
  4 │   list
    │   ^^^^

  Modules are not values, so you cannot assign them to variables, pass them to
  functions, or anything else that you would do with a value.
  ```

  ([sobolevn](https://github.com/sobolevn))

- An helpful error message has been added when the programmer attempts to write
  a function within a custom type definition, likely trying to declare an OOP
  class. For example:

  ```rakun
  pub record User {
    User(name: String)

    fn greet(user: User) -> String {
      "hello " ++ user.name
    }
  }
  ```

  Now results in the following error:

  ```txt
  error: Syntax error
    ┌─ /Users/giacomocavalieri/Desktop/prova/src/prova.rakun:8:3
    │
  8 │   fn greet(user: User) -> String {
    │   ^^ I was not expecting this

  Found the keyword `fn`, expected one of:
  - `}`
  - a record constructor
  Hint: Rakun is not an object oriented programming language so
  functions are declared separately from types.
  ```

  ([sobolevn](https://github.com/sobolevn))

- The compiler now gives a hint to import a module when accessing modules that
  aren't imported. It only suggests a module if it exports a type/value with
  the same name as what the user was trying to access:

  ```rakun
  pub fn main() {
    io.println("Hello, world!")
  }
  ```

  Produces the following error:

  ```
  error: Unknown module
    ┌─ /src/file.rakun:2:3
    │
  2 │   io.println("Hello, world!")
    │   ^^

  No module has been found with the name `io`.
  Hint: Did you mean to import `rakun/io`?
  ```

  This code, however, produces no hint:

  ```rakun
  pub fn main() {
    io.non_existent()
  }
  ```

  ([Surya Rose](https://github.com/gearsdatapacks))

- The compiler now provides improved suggestions in the error for an
  inexhaustive case expression. The following code:

    ```rakun
    let a = True
    case a {}
    ```

  Now produces this error:

    ```
    error: Inexhaustive patterns
      ┌─ /src/file.rakun:3:3
      │
    3 │   case a {}
      │   ^^^^^^^^^


  This case expression does not have a pattern for all possible values. If it
  is run on one of the values without a pattern then it will crash.

  The missing patterns are:

      False
      True
  ```

  Whereas before, it would suggest `_` as the only missing pattern.
  ([Surya Rose](https://github.com/GearsDatapacks))

- Improve error message for using `@external` with unknown target
  ([Jiangda Wang](https://github.com/frank-iii))

- Improved error title when using an unknown module value.
  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

- The compiler now shows an helpful error message if you try writing an `if`
  expression instead of a case. For example, this code:

  ```rakun
  pub fn main() {
    let a = if wibble {
      1
    }
  }
  ```

  Results in the following error:

  ```txt
  error: Syntax error
    ┌─ /src/parse/error.rakun:3:11
    │
  3 │   let a = if wibble {
    │           ^^ Rakun doesn't have if expressions

  If you want to write a conditional expression you can use a `case`:

      case condition {
        True -> todo
        False -> todo
      }

  See: https://tour.rakun.run/flow-control/case-expressions/
  ```

  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

### Formatter

- The formatter now adds a `todo` after a `use` expression if it is the last
  expression in a block. For example, the following code:

  ```rakun
  pub fn main() {
    use user <- result.try(fetch_user())
  }
  ```

  Is rewritten as:

  ```rakun
  pub fn main() {
    use user <- result.try(fetch_user())
    todo
  }
  ```

  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

### Language Server

- The language server can now suggest a code action to assign an unused value to
  `_`.
  ([Jiangda Wang](https://github.com/frank-iii))

- The language server can now suggest a code action to import modules for
  existing code which references unimported modules:

  ```rakun
  pub fn main() {
    io.println("Hello, world!")
  }
  ```

  Becomes:

  ```rakun
  import rakun/io

  pub fn main() {
    io.println("Hello, world!")
  }
  ```

  ([Surya Rose](https://github.com/gearsdatapacks))

- The Language Server can now suggest a code action to fill in the missing
  patterns of a case expression:

  ```rakun
  let a = True
  case a {}
  ```

  Becomes:

  ```rakun
  let a = True
  case a {
    False -> todo
    True -> todo
  }
  ```

  ([Surya Rose](https://github.com/GearsDatapacks))

### Bug Fixes

- Fixed a bug where the warnings were printed above the errors without any new
  line between them.
  ([Victor Kobinski](https://github.com/vkobinski))

- Fixed a bug which caused the language server and compiler to crash when two
  constructors of the same name were created.
  ([Surya Rose](https://github.com/GearsDatapacks))

- Fixed a bug where jumping to the definition of an unqualified function would
  produce the correct location, but remain in the same file.
  ([Surya Rose](https://github.com/gearsdatapacks))

- Fixed a bug where incorrect syntax error message were shown, when using `:` or
  `=` in wrong positions in expressions.
  ([Ankit Goel](https://github.com/crazymerlyn))

- Fixed a bug where the compiler would crash when pattern matching on a type
  which had constructors of duplicate names.
  ([Surya Rose](https://github.com/gearsdatapacks))

- Fixed a bug where referencing record constructors in JavaScript constants but
  not calling them could produce invalid code.
  ([Louis Pilfold](https://github.com/lpil))

- Fixed a bug where source links in HTML documentation would be incorrect for
  Codeberg, SourceHut, and Gitea.
  ([sobolevn](https://github.com/sobolevn))

- Fixed a bug with Erlang code generation for discard utf8 patterns in bit
  arrays.
  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

- Fixed a bug which affected inference of function calls in pipe expressions.
  ([sobolevn](https://github.com/sobolevn))

- Improved an error message when using variable names starting with an
  underscore in expression like: `let some = _func()` or `case { 1 -> _func() }`
  ([sobolevn](https://github.com/sobolevn))

- Fixed a bug where the provided `REBAR_BARE_COMPILER_OUTPUT_DIR` env var would
  use relative path instead of absolute path causing compilation errors in some
  packages.
  ([Gustavo Inacio](https://github.com/gusinacio))

- Fixed a bug where the compiler would print incorrect missing patterns for
  inexhaustive case expressions matching on more than one subject.
  ([Surya Rose](https://github.com/GearsDatapacks))

- Fixed a bug where the compiler would not check the target support of a
  function if it was imported and not used, and generate invalid code.
  ([Surya Rose](https://github.com/GearsDatapacks))

- Fixed a bug where an qualified unused constructor wouldn't be reported as
  unused.
  ([Giacomo Cavalieri](https://github.com/giacomocavalieri))

- The Language Server now correctly shows completions for values in the Rakun
  prelude.
  ([Surya Rose](https://github.com/GearsDatapacks))

## v1.4.1 - 2024-08-04

### Bug Fixes

- Fix a bug that caused record accessors for private types to not be completed
  by the LSP, even when in the same module.
  ([Ameen Radwan](https://github.com/Acepie))
