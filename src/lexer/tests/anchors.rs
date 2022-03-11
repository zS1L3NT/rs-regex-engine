use super::*;
use crate::valid;

#[test]
fn anchors() {
    valid!("/^/", 1);
    valid!("/^$/", 2);
    valid!("/^a$/", vec![AnchorStart(0), Literal('a', 1), AnchorEnd(2)]);
}
