<!-- 
Please don't edit. This document has been generated from src/readme.tpl.md
--> 
# X-Path

- [X-Path](#x-path)
- [Use cases](#use-cases)
    - [Config files](#config-files)
    - [Clear expectations](#clear-expectations)
    - [Readable and Testable](#readable-and-testable)
    - [Cross-platform](#cross-platform)
    - [Convenient](#convenient)
- [Design goals](#design-goals)
- [Limitations](#limitations)
    - [Characters](#characters)
    - [Path separators (slash) and drives](#path-separators-(slash)-and-drives)
    - [Path components](#path-components)
    - [Filenames](#filenames)
- [Path resolution](#path-resolution)
- [Environment variables](#environment-variables)
- [Path comparison](#path-comparison)
- [References](#references)


> <span style="color:darkorange">**⚠️ WARNING**</span>
>
> This is work in progress and is not ready for use


# Use cases

## Config files

The paths below are valid on any platform. They will be cleaned
and have environment variables resolved at load.

```toml
dir1 = "~/mydir/${SOME_ENV}/../"
dir2 = "c:\\anotherdir\\%ANOTHER_ENV%"
```

## Clear expectations

Use one of the below to communicate what your function or API expects.

|     | Any       | Dir          | File          |
| --- | ---       | ---          | ---           |
| Any | [AnyPath] | [DirPath]    | [FilePath]    |
| Rel | [RelPath] | [RelDirPath] | [RelFilePath] |
| Abs | [AbsPath] | [AbsDirPath] | [AbsFilePath] |
 
```rust
fn mirror(file: RelFilePath, from: AbsDirPath, to: AbsDirPath) {}
```

## Readable and Testable

The [Display] implementation outputs the platform-native representation of
a path, using the native path separator whereas the [Debug] implementation
uses the `/` path separator and also includes the path type.
Both for ease of testing.

By default, the paths are contracted, meaning that if the path starts with
user home dir then the former that part is replaced with `~` and if it starts
with the current working directory the replacement is `.`.

```rust
#[test]
fn test() -> anyhow::Result<()> {
    // imagine that the path string is read from a conf.toml file:
    let dir = AbsDirPath::new(r"~/dir1//..\dir2");
    
    //////// Display ////////

    #[cfg(not(win))]
    assert_eq!(format!("{dir}"), "~/dir2");
    #[cfg(win)]
    assert_eq!(format!("{dir}"), r"~\dir2");

    // using alternate
    #[cfg(not(win))]
    assert_eq!(format!("{dir:#}"), "/home/user/dir2");
    #[cfg(win)]
    assert_eq!(format!("{dir:#}"), r"c:\Users\user\dir2");

    //////// Debug ////////
    
    // using standard Debug
    assert_eq!(format!("{dir:?}"), r#"AbsDirPath("~/dir2")"#);

    // using alternative Debug
    assert_eq!(format!("{dir:#?}", r#"AbsDirPath("/home/user/dir2")"#))
}
```

## Cross-platform

Both Windows-style and Unix-style paths can be used on all platforms. They
are all resolved and converted into a unified format that is comparable.

The typical file system restrictions are enforced when read.
On Windows, the NTFS, VFAT and exFAT restrictions are applied which are
much more stringent than the Unix ones. Enable the feature `strict` if
you want the same restrictions applied when running on Unix.

## Convenient

Access the paths as `&str`, all paths implement:
- [Display](std::fmt::Display) for easy display.
- `AsRef<Path>` for interoperability with all the [std::fs] operations.
- Iterate through all the path segments as `&str`ings with `path.segments()`.
- Many convenient functions: see the doc for each path type.

# Design goals

- Make rust's typical _"if it compiles it works"_ experience work for cross-platform path handling as well.
- Make Paths comparable, i.e. they are resolved to a common format in memory, and converted to
  a platform-specific format when used.
- Write config files using paths that work across platforms (as far as possible).
- AnyPath for general use and specific ones when you need to assure that
- Provide types distinguishing between Absolute or Relative and Directory or File:
    - FilePath, FileAbsPath, FileRelPath
    - DirPath, DirAbsPath, DirAbsPath
- Support for the major operating systems and file systems:
    - Linux & Unix: most file systems.
    - macOS: HFS+, APFS.
    - Windows: exFAT, NTFS. With feature `strict` enabled.
- Comparable paths (because they are resolved, see [Path Comparison](#path-comparison) below).

Non-goals:
- Maximum performance.
- Crazy filenames. I.e. only UTF-8 filenames are supported.

Other:
- Displays resolved paths or use `.native_string()` or `format("{path:#}")` for outputting OS native string.
- Error:
    - handling with [anyhow](https://crates.io/crates/anyhow) aims to produce comprehensive
      human-readable messages instead of machine-parsable ones.
    - the message always includes the path in question.
    - the message includes the current working directory for relative paths.


# Limitations

The limits are verified when creating and manipulating a path. By default, on Unix-based platforms,
only a few limits are applied. On Windows, there are automatically more restrictions.

If you want to ensure that the paths work seamlessly (as far as possible)
on all platforms (i.e. paths authored on Linux work on Windows) then turn on the `strict`
Cargo feature.

## Characters

Reserved characters:
- Slash (`/` and `\`): are used as path separators on all platforms.
- `$` and `%`: when at the start of a path or immediately after a slash it will be
  interpreted as an environment variable see section [Environment variables](#environment-variables)
- `.` and `~` when at the start of a path followed by either a slash or nothing are
  interpreted as the current working dir and user home dir respectively.

Always forbidden:
- Non UTF-8 characters (i.e. don't use [OsStr](std::ffi::OsStr) or [OsString](std::ffi::OsString))
- NULL, `:`

Forbidden in `strict` mode or when running on Windows:
- Ascii control characters: 0x00-0x1F, 0x7F
- `"`, `*`, `/`, `<`, `>`, `?`, `\`, `|`
- Filenames: CON, PRN, AUX, NUL, COM0 - COM9 and LPT0 - LPT9. Also any of these filenames
  followed by an extension (ex: .txt).

## Path separators (slash) and drives

The path separators are kept in memory and displayed in a platform-native representation,
i.e. using the platform where the binary is running. For Windows, it's `\` and for the others `/`.

On Windows, any drive letters are kept lower-cased, and on the others, it is discarded.

This means that a string written as either `c:\my\path` or `/my/path`
is converted and stored in memory and displayed as:
- Windows: `c:\my\path` when the current directory's drive letter is `c`
- Others: `/my/path`

## Path components

Path components are limited to a maximum of 255 characters.

## Filenames

Forbidden in `strict` mode or when running on Windows: CON, PRN, AUX, NUL, COM0 - COM9 and LPT0 - LPT9.
Also any of these filenames followed by an extension (ex: .txt).

# Path resolution

Path resolution is done without file-system access so that paths don't need to exist.

| Path<sup>*</sup>         | Becomes                                  | When               | Is                                       | Comment
| ---                      | ---                                      | ---                | ---                                      | ---
| `.`, `./`                | nix: `/tmp`<br>win: `c:\tmp`             | current_dir()      | nix: `/tmp`<br>win: `c:\tmp`             |
| `~`, `~/`                | nix: `/Users/tom`<br>win: `c:\Users\tom` | home_dir()         | nix: `/Users/tom`<br>win: `c:\Users\tom` |
| `/`                      | nix: `/`<br>win: `c:\`                   | -<br>current_dir() | - <br>win: `c:/somedir`                  | - <br> win: Same drive as the current dir
| `c:/`, `C:/`             | nix: `/`<br>win: `c:\`                   |                    |                                          | nix: Drive letter removed<br>win: Drive letters always in lower case
| `c:dir`                  | nix: `/tmp/dir`<br>win: `c:\tmp\dir`     | current_dir()      | nix: `/tmp`<br>win: `c:\tmp`             |
| `dir//dir`               | nix: `dir/dir`<br>win: `dir\dir`         |                    |                                          | Multiple slashes are joined
| `dir/./dir`              | nix: `dir/dir`<br>win: `dir\dir`         |                    |                                          | Dots inside of a path are ignored
| `dir/..`                 |                                          |                    |                                          | Empty path
| `dir1/dir2/..`           | `dir1`                                   |                    |                                          |
| `${MYDIR}`,<br>`%MYDIR%` | `dir`                                    | var("MYDIR")       | `dir`                                    | See [Environment variables](#environment-variables)

Legend:
- <sup>*</sup> - Any `/` can also be `\`.
- nix - Unix-based platforms: Linux, Unix, macOS.
- win - Windows
- current_dir() - refers to rust's [std::env::current_dir()](https://doc.rust-lang.org/std/env/fn.current_dir.html)
- var() - refers to rust's [std::env::var(key)](https://doc.rust-lang.org/std/env/fn.var.html)
- home_dir() - refers to the [dirs_sys::home_dir()](https://docs.rs/dirs-sys/0.4.0/dirs_sys/fn.home_dir.html)

# Environment variables

There is restricted support for environment variables where only a path segment that
in Unix style: starts with `${` and ends with `}` or in Windows style starts and ends with `%`
is interpreted as an environment variable and expanded when read. The stricter-than-usual
requirements reduce interference with normal paths.

Interpreted as environment variables:
- `/dir/${MYVAR}/`, `${MYVAR}`, `${MYVAR}/dir`, `/dir/${MYVAR}`
- `/dir/%MYVAR%/`, `%MYVAR%`, `%MYVAR%/dir`, `/dir/%MYVAR%`

Not interpreted as environment vars:
- `$MYVAR` - missing curly braces
- `hi${MYVAR}`, `${MYVAR}hi`, `hi%MYVAR%`, `%MYVAR%hi` - any character before or after that is not a slash.
- `${MYVAR`, `%MYVAR` - not closed.
- `${MY-VAR}`, `%MY-VAR%`: use of character not permitted in environment variables.

Returns an error:
- `${}`, `\${}`, `\${}\` - empty keys are invalid
- `%MYVAR` when the environment variable MYVAR is not defined.

# Path comparison

While paths preserve casing when kept in memory comparing is done in a case-insensitive manner.

# References

- [File path formats on Windows systems](https://learn.microsoft.com/en-us/dotnet/standard/io/file-path-formats)
- [Naming Files, Paths, and Namespaces](https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file)
- [Wikipedia: Filenames - Comparison of filename limitations](https://en.wikipedia.org/wiki/Filename#Comparison_of_filename_limitations)
