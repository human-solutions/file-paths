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
    - `.literal(..)` if you really need to skip resolution.
- Path resolution without file-system access so that paths don't need to exist.
- Comparable paths (because they are resolved, see above).
- Error handling with [anyhow](https://crates.io/crates/anyhow) aims to produce comprehensive human-readable messages instead of machine-parsable ones.
- Support for the major operating systems: Linux, Unix, macOS and Windows.

Non-goals:
- Maximum performance.
- Crazy filenames. I.e. only UTF-8 filenames are supported.

