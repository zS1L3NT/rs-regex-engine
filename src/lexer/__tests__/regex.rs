use super::*;
use crate::{invalid, valid};

#[test]
fn regex_start() {
    valid!("/abc/", 3);
    valid!("/bcd/", 3);
    invalid!("/abc/", "Expected RegExp to start with a </>");
    invalid!("/bcd/", "Expected RegExp to start with a </>");
}

#[test]
fn regex_end() {
    valid!("/abc/", 3);
    valid!("/bcd/", 3);
    invalid!("/abc/", "Expected RegExp to start with a </>");
    invalid!("/bcd/", "Expected RegExp to start with a </>");
}

#[test]
fn length_check() {
    valid!("//", 0);
    valid!("/abc/", 3);
    valid!("/^abc$/", 5);
    valid!("/abc{2,5}/", 3);
    valid!("/a(b|c)[^d]/", 9);
    valid!("/\\ba\\b", 3);
}
