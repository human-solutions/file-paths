<!-- 
Please don't edit. This document has been generated from src/readme.tpl.md
--> 
Design goals:
- Make rust's typical _"if it compiles it works"_ experience work for path handling as well.
- Types distinguishing between Absolute or Relative and Directory or File:
    - FilePath, AbsFilePath, RelFilePath
    - DirPath, AbsDirPath, AbsDirPath
- Automatically resolved paths. I.e:
    - `./folder1/folder2/../file.txt` becomes `./folder1/file.txt`
    - `~/folder1` becomes `/<user-home-dir>/folder1`.
    - `./folder1/$MY_DIR` becomes `./folder1/dir1` when `$MY_DIR` is `dir1`
    - `folder1` becomes `./folder1`
    - `.*_lit(..)` functions if you really need to skip resolution and create a path like `./folder1/"one/name"/"$HOME"`
    - `c:\\folder1` becomes `c:/folder1`
- Path resolution without file-system access so that paths don't need to exist.
- Comparable paths (because they are resolved, see above).
- Displays resolved paths or use `.native_string()` or `format("{path:#}")` for outputting OS native string.
- Error:
    - handling with [anyhow](https://crates.io/crates/anyhow) aims to produce comprehensive human-readable messages instead of machine-parsable ones.
    - the message always includes the path in question.
    - the message includes the current working directory for relative paths.
- Support for the major operating systems: Linux, Unix, macOS and Windows.

Non-goals:
- Maximum performance.
- Crazy filenames. I.e. only UTF-8 filenames are supported.

