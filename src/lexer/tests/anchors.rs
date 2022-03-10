use super::*;
use crate::valid;

#[test]
fn anchors() {
    valid!("/^/", 1);
    valid!("/^$/", 2);
    valid!("/^a$/", vec![AssertStart(0), Literal('a', 1), AssertEnd(2)]);
}
