// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod reader;
mod validator;

pub use validator::{EcmaRegexValidator, EcmaVersion};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_flags() {
        let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_flags("gimuys"), Ok(()));
        assert_eq!(validator.validate_flags("gimuy"), Ok(()));
        assert_eq!(validator.validate_flags("gim"), Ok(()));
        assert_eq!(validator.validate_flags("g"), Ok(()));
        assert_eq!(validator.validate_flags("i"), Ok(()));
        assert_eq!(validator.validate_flags("m"), Ok(()));
        assert_eq!(validator.validate_flags("s"), Ok(()));
        assert_eq!(validator.validate_flags("u"), Ok(()));
        assert_eq!(validator.validate_flags("y"), Ok(()));

        assert_eq!(validator.validate_flags("gy"), Ok(()));
        assert_eq!(validator.validate_flags("iy"), Ok(()));
        assert_eq!(validator.validate_flags("my"), Ok(()));
        assert_eq!(validator.validate_flags("uy"), Ok(()));
    }

    #[test]
    fn duplicate_flags() {
        let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_flags("gimgu"), Err("Duplicated flag g".to_string()));
        assert_eq!(validator.validate_flags("migg"), Err("Duplicated flag g".to_string()));
        assert_eq!(validator.validate_flags("ii"), Err("Duplicated flag i".to_string()));
        assert_eq!(validator.validate_flags("mm"), Err("Duplicated flag m".to_string()));
        assert_eq!(validator.validate_flags("ss"), Err("Duplicated flag s".to_string()));
        assert_eq!(validator.validate_flags("uu"), Err("Duplicated flag u".to_string()));
        assert_eq!(validator.validate_flags("yy"), Err("Duplicated flag y".to_string()));
    }

    #[test]
    fn invalid_flags() {
        let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_flags("gimuf"), Err("Invalid flag f".to_string()));
        assert_eq!(validator.validate_flags("a"), Err("Invalid flag a".to_string()));
    }

    #[test]
    fn validate_pattern_test() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_pattern("[abc]de|fg", false), Ok(()));
        assert_eq!(validator.validate_pattern("[abc]de|fg", true), Ok(()));
        assert_eq!(validator.validate_pattern("^.$", false), Ok(()));
        assert_eq!(validator.validate_pattern("^.$", true), Ok(()));
        assert_eq!(validator.validate_pattern("foo\\[bar", false), Ok(()));
        assert_eq!(validator.validate_pattern("foo\\[bar", true), Ok(()));

        assert_ne!(validator.validate_pattern("^[z-a]$", false), Ok(()));
        assert_ne!(validator.validate_pattern("0{2,1}", false), Ok(()));
        assert_ne!(validator.validate_pattern("\\", false), Ok(()));
        assert_ne!(validator.validate_pattern("a**", false), Ok(()));
        assert_ne!(validator.validate_pattern("++a", false), Ok(()));
        assert_ne!(validator.validate_pattern("?a", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{1}{1,}", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{1,2}{1}", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{1,}{1}", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{0,1}{1,}", false), Ok(()));
        assert_ne!(validator.validate_pattern("a***", false), Ok(()));
        assert_ne!(validator.validate_pattern("a++", false), Ok(()));
        assert_ne!(validator.validate_pattern("a+++", false), Ok(()));
        assert_ne!(validator.validate_pattern("a???", false), Ok(()));
        assert_ne!(validator.validate_pattern("a????", false), Ok(()));
        assert_ne!(validator.validate_pattern("*a", false), Ok(()));
        assert_ne!(validator.validate_pattern("**a", false), Ok(()));
        assert_ne!(validator.validate_pattern("+a", false), Ok(()));
    }

    #[test]
    fn unicode_quantifier_without_atom() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_ne!(validator.validate_pattern("*", true), Ok(()));
        assert_ne!(validator.validate_pattern("+", true), Ok(()));
        assert_ne!(validator.validate_pattern("?", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1}", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1,}", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1,2}", true), Ok(()));

        assert_ne!(validator.validate_pattern("*?", true), Ok(()));
        assert_ne!(validator.validate_pattern("+?", true), Ok(()));
        assert_ne!(validator.validate_pattern("??", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1}?", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1,}?", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1,2}?", true), Ok(()));
    }

    #[test]
    fn unicode_incomplete_quantifier() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_ne!(validator.validate_pattern("a{", true), Ok(()));
        assert_ne!(validator.validate_pattern("a{1", true), Ok(()));
        assert_ne!(validator.validate_pattern("a{1,", true), Ok(()));
        assert_ne!(validator.validate_pattern("a{1,2", true), Ok(()));

        assert_ne!(validator.validate_pattern("{", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1,", true), Ok(()));
        assert_ne!(validator.validate_pattern("{1,2", true), Ok(()));
    }

    #[test]
    fn unicode_single_bracket() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_ne!(validator.validate_pattern("(", true), Ok(()));
        assert_ne!(validator.validate_pattern(")", true), Ok(()));
        assert_ne!(validator.validate_pattern("[", true), Ok(()));
        assert_ne!(validator.validate_pattern("]", true), Ok(()));
        assert_ne!(validator.validate_pattern("{", true), Ok(()));
        assert_ne!(validator.validate_pattern("}", true), Ok(()));
    }

    #[test]
    fn unicode_escapes() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_pattern("\\u{10ffff}", true), Ok(()));
        assert_ne!(validator.validate_pattern("\\u{110000}", true), Ok(()));
        assert_eq!(validator.validate_pattern("\\u{110000}", false), Ok(()));
        assert_eq!(validator.validate_pattern("foo\\ud803\\ude6dbar", true), Ok(()));
        assert_eq!(validator.validate_pattern("(\u{12345}|\u{23456}).\\1", true), Ok(()));
        assert_eq!(validator.validate_pattern("\u{12345}{3}", true), Ok(()));

        // unicode escapes in character classes
        assert_eq!(validator.validate_pattern("[\\u0062-\\u0066]oo", false), Ok(()));
        assert_eq!(validator.validate_pattern("[\\u0062-\\u0066]oo", true), Ok(()));
        assert_eq!(validator.validate_pattern("[\\u{0062}-\\u{0066}]oo", true), Ok(()));
        assert_eq!(validator.validate_pattern("[\\u{62}-\\u{00000066}]oo", true), Ok(()));

        // invalid escapes
        assert_eq!(validator.validate_pattern("first\\u\\x\\z\\8\\9second", false), Ok(()));
        assert_eq!(validator.validate_pattern("[\\u\\x\\z\\8\\9]", false), Ok(()));
        assert_ne!(validator.validate_pattern("/\\u/u", true), Ok(()));
        assert_ne!(validator.validate_pattern("/\\u12/u", true), Ok(()));
        assert_ne!(validator.validate_pattern("/\\ufoo/u", true), Ok(()));
        assert_ne!(validator.validate_pattern("/\\x/u", true), Ok(()));
        assert_ne!(validator.validate_pattern("/\\xfoo/u", true), Ok(()));
        assert_ne!(validator.validate_pattern("/\\z/u", true), Ok(()));
        assert_ne!(validator.validate_pattern("/\\8/u", true), Ok(()));
        assert_ne!(validator.validate_pattern("/\\9/u", true), Ok(()));
    }
}
