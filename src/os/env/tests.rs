use std::borrow::Cow;

use super::expand;
use crate::os::{LinTestOS, WinTestOS};

#[test]
fn expand_envs_win() {
    assert_eq!(exp_ok_win("$HI"), "$HI");

    assert_eq!(exp_ok_win("${HI}"), "=hi=");
    assert_eq!(exp_ok_win("\\${HI}"), "\\=hi=");
    assert_eq!(exp_ok_win("\\${HI}\\"), "\\=hi=\\");

    assert_eq!(exp_ok_win("%HI%"), "=hi=");
    assert_eq!(exp_ok_win("\\%HI%"), "\\=hi=");
    assert_eq!(exp_ok_win("\\%HI%\\"), "\\=hi=\\");

    // not expanded
    assert_eq!(exp_ok_win("\\s$HI$"), "\\s$HI$");
    assert_eq!(exp_ok_win("\\%$HI"), "\\%$HI");
    assert_eq!(exp_ok_win("\\${HI"), "\\${HI");
    assert_eq!(exp_ok_win("\\${H-}"), "\\${H-}");
    assert_eq!(exp_ok_win("\\${H}s"), "\\${H}s");
    assert_eq!(exp_ok_win("\\%H%s"), "\\%H%s");
    assert_eq!(exp_ok_win("\\$"), "\\$");

    assert_eq!(exp_ok_win("\\dir1\\.\\dir2"), "\\dir1\\.\\dir2");
    assert_eq!(exp_ok_win("dir1\\.\\dir2"), "dir1\\.\\dir2");

    assert_eq!(exp_ok_win("\\dir1\\~\\dir2"), "\\dir1\\~\\dir2");
    assert_eq!(exp_ok_win("dir1\\~\\dir2"), "dir1\\~\\dir2");

    // errors
    assert_eq!(
        exp_err_win("\\%%"),
        "empty environment variable in path: \\%%"
    );

    assert_eq!(
        exp_err_win("\\${}"),
        "empty environment variable in path: \\${}"
    );

    assert_eq!(
        exp_err_win("\\${FAIL}"),
        "environment variable 'FAIL' is not defined"
    );

    assert_eq!(exp_ok_win("."), "C:\\current\\");
    assert_eq!(exp_ok_win(".\\"), "C:\\current\\");
    assert_eq!(exp_ok_win(".\\di"), "C:\\current\\di");

    assert_eq!(exp_ok_win("~"), "C:\\User\\test\\");
    // the double ending slash is ok at expansion stage as it is removed later
    assert_eq!(exp_ok_win("~\\"), "C:\\User\\test\\\\");
    assert_eq!(exp_ok_win("~\\di"), "C:\\User\\test\\\\di");
}

#[cfg(test)]
fn exp_ok_win(path: &str) -> Cow<str> {
    expand::<WinTestOS>(path).unwrap()
}

#[cfg(test)]
fn exp_err_win(path: &str) -> String {
    expand::<WinTestOS>(path).unwrap_err().to_string()
}

#[test]
fn exp_envs_lin() {
    assert_eq!(exp_ok_lin("$HI"), "$HI");

    assert_eq!(exp_ok_lin("${HI}"), "=hi=");
    assert_eq!(exp_ok_lin("/${HI}"), "/=hi=");
    assert_eq!(exp_ok_lin("/${HI}/"), "/=hi=/");

    assert_eq!(exp_ok_lin("%HI%"), "=hi=");
    assert_eq!(exp_ok_lin("/%HI%"), "/=hi=");
    assert_eq!(exp_ok_lin("/%HI%/"), "/=hi=/");

    // not expanded
    assert_eq!(exp_ok_lin("/s$HI$"), "/s$HI$");
    assert_eq!(exp_ok_lin("/%$HI"), "/%$HI");
    assert_eq!(exp_ok_lin("/${HI"), "/${HI");
    assert_eq!(exp_ok_lin("/${H-}"), "/${H-}");
    assert_eq!(exp_ok_lin("/${H}s"), "/${H}s");
    assert_eq!(exp_ok_lin("/%H%s"), "/%H%s");
    assert_eq!(exp_ok_lin("/$"), "/$");

    assert_eq!(exp_ok_lin("/dir1/./dir2"), "/dir1/./dir2");
    assert_eq!(exp_ok_lin("dir1/./dir2"), "dir1/./dir2");

    assert_eq!(exp_ok_lin("/dir1/~/dir2"), "/dir1/~/dir2");
    assert_eq!(exp_ok_lin("dir1/~/dir2"), "dir1/~/dir2");

    // errors
    assert_eq!(
        exp_err_lin("/%%"),
        "empty environment variable in path: /%%"
    );

    assert_eq!(
        exp_err_lin("/${}"),
        "empty environment variable in path: /${}"
    );

    assert_eq!(
        exp_err_lin("/${FAIL}"),
        "environment variable 'FAIL' is not defined"
    );

    assert_eq!(exp_ok_lin("."), "/var/test/");
    assert_eq!(exp_ok_lin("./"), "/var/test/");
    assert_eq!(exp_ok_lin("./dir"), "/var/test/dir");

    assert_eq!(exp_ok_lin("~"), "/home/test/");
    assert_eq!(exp_ok_lin("~/"), "/home/test/");
    assert_eq!(exp_ok_lin("~/dir"), "/home/test/dir");
}

fn exp_ok_lin(path: &str) -> Cow<str> {
    expand::<LinTestOS>(path).unwrap()
}

fn exp_err_lin(path: &str) -> String {
    expand::<LinTestOS>(path)
        .unwrap_err()
        .to_string()
        .replace('\\', "/")
}
