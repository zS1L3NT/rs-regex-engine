mod asterisk {
    use super::super::*;
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/*/", "The preceding token is not quantifiable");
    }
    #[test]
    fn with_literal() {
        valid!("/a*/", vec![Literal('a', 0), Quantifier(Asterisk, 1)]);
    }
}

mod plus {
    use super::super::*;
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/+/", "The preceding token is not quantifiable");
    }
    #[test]
    fn with_literal() {
        valid!("/a+/", vec![Literal('a', 0), Quantifier(Plus, 1)]);
    }
}

mod question_mark {
    use super::super::*;
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/?/", "The preceding token is not quantifiable");
    }
    #[test]
    fn with_literal() {
        valid!("/a?/", vec![Literal('a', 0), Quantifier(QuestionMark, 1)]);
    }
}

mod count {
    use super::super::*;
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/{1}/", "The preceding token is not quantifiable");
    }

    #[test]
    fn as_literals() {
        valid!("/a{}/", 3);
    }

    #[test]
    fn with_literal() {
        valid!("/a{1}/", vec![Literal('a', 0), Quantifier(Count(1), 1)]);
    }
}

mod range {
    use super::super::*;
    use crate::{invalid, valid};

    #[test]
    fn invalid() {
        invalid!("/{1,2}/", "The preceding token is not quantifiable");
    }

    #[test]
    fn as_literals() {
        valid!("/a{,}/", 4);
    }

    #[test]
    fn only_start() {
        valid!(
            "/a{1,}/",
            vec![Literal('a', 0), Quantifier(Range(Some(1), None), 1)]
        );
    }

    #[test]
    fn only_end() {
        valid!(
            "/a{,1}",
            vec![Literal('a', 0), Quantifier(Range(None, Some(1)), 1)]
        );
    }

    #[test]
    fn both() {
        valid!(
            "/a{1,2}/",
            vec![Literal('a', 0), Quantifier(Range(Some(1), Some(2)), 1)]
        );
    }

    #[test]
    fn nested_brackets() {
        valid!("/a{{1,}}/", 4);
    }
}
