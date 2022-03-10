use super::*;
use crate::valid;

#[test]
fn escape() {
    valid!("/\\./", vec![Literal('.', 0)]);
    valid!("/\\+/", vec![Literal('+', 0)]);
    valid!("/\\*/", vec![Literal('*', 0)]);
    valid!("/\\?/", vec![Literal('?', 0)]);
    valid!("/\\^/", vec![Literal('^', 0)]);
    valid!("/\\$/", vec![Literal('$', 0)]);
    valid!("/\\(/", vec![Literal('(', 0)]);
    valid!("/\\)/", vec![Literal(')', 0)]);
    valid!("/\\[/", vec![Literal('[', 0)]);
    valid!("/\\]/", vec![Literal(']', 0)]);
    valid!("/\\{/", vec![Literal('{', 0)]);
    valid!("/\\}/", vec![Literal('}', 0)]);
    valid!("/\\|/", vec![Literal('|', 0)]);
    valid!("/\\\\/", vec![Literal('\\', 0)]);
}