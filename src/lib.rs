// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

#[macro_use]
extern crate lazy_static;

mod reader;
mod unicode;
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
        assert_eq!(validator.validate_flags("igi"), Err("Duplicated flag i".to_string()));

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
        assert_eq!(validator.validate_flags("gI"), Err("Invalid flag I".to_string()));
        assert_eq!(validator.validate_flags("a"), Err("Invalid flag a".to_string()));
        assert_eq!(validator.validate_flags("1"), Err("Invalid flag 1".to_string()));
    }

    #[test]
    fn validate_pattern_test() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_pattern("", false), Ok(()));
        assert_eq!(validator.validate_pattern("[abc]de|fg", false), Ok(()));
        assert_eq!(validator.validate_pattern("[abc]de|fg", true), Ok(()));
        assert_eq!(validator.validate_pattern("^.$", false), Ok(()));
        assert_eq!(validator.validate_pattern("^.$", true), Ok(()));
        assert_eq!(validator.validate_pattern("foo\\[bar", false), Ok(()));
        assert_eq!(validator.validate_pattern("foo\\[bar", true), Ok(()));
        assert_eq!(validator.validate_pattern("\\w+\\s", false), Ok(()));
        assert_eq!(validator.validate_pattern("(\\w+), (\\w+)", false), Ok(()));
        assert_eq!(validator.validate_pattern("\\/\\/.*|\\/\\*[^]*\\*\\/", false), Ok(()));
        assert_eq!(validator.validate_pattern("(\\d{1,2})-(\\d{1,2})-(\\d{4})", false), Ok(()));
        assert_eq!(validator.validate_pattern("(?:\\d{3}|\\(\\d{3}\\))([-\\/\\.])\\d{3}\\1\\d{4}", false), Ok(()));
        assert_eq!(validator.validate_pattern("https?:\\/\\/(www\\.)?[-a-zA-Z0-9@:%._\\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\\+.~#?&//=]*)", false), Ok(()));

        //assert_eq!(validator.validate_pattern("\\p{Script=Greek}", true), Ok(()));
        //assert_eq!(validator.validate_pattern("\\p{Alphabetic}", true), Ok(()));

        assert_ne!(validator.validate_pattern("\\", false), Ok(()));
        assert_ne!(validator.validate_pattern("a**", false), Ok(()));
        assert_ne!(validator.validate_pattern("++a", false), Ok(()));
        assert_ne!(validator.validate_pattern("?a", false), Ok(()));
        assert_ne!(validator.validate_pattern("a***", false), Ok(()));
        assert_ne!(validator.validate_pattern("a++", false), Ok(()));
        assert_ne!(validator.validate_pattern("a+++", false), Ok(()));
        assert_ne!(validator.validate_pattern("a???", false), Ok(()));
        assert_ne!(validator.validate_pattern("a????", false), Ok(()));
        assert_ne!(validator.validate_pattern("*a", false), Ok(()));
        assert_ne!(validator.validate_pattern("**a", false), Ok(()));
        assert_ne!(validator.validate_pattern("+a", false), Ok(()));
        assert_ne!(validator.validate_pattern("[{-z]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[a--z]", false), Ok(()));

        assert_ne!(validator.validate_pattern("0{2,1}", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{1}{1,}", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{1,2}{1}", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{1,}{1}", false), Ok(()));
        assert_ne!(validator.validate_pattern("x{0,1}{1,}", false), Ok(()));

        assert_ne!(validator.validate_pattern("\\1(\\P{P\0[}()/", true), Ok(()));
    }

    #[test]
    fn character_range_order() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_ne!(validator.validate_pattern("^[z-a]$", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-ac-e]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[c-eb-a]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[a-dc-b]", false), Ok(()));

        assert_ne!(validator.validate_pattern("[\\10b-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\ad-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\bd-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\Bd-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\db-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\Db-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\sb-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\Sb-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\wb-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\Wb-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\0b-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\td-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\nd-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\vd-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\fd-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\rd-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\c0001d-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\x0061d-G]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[\\u0061d-G]", false), Ok(()));

        assert_ne!(validator.validate_pattern("[b-G\\10]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\a]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\b]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\B]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-G\\d]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-G\\D]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-G\\s]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-G\\S]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-G\\w]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-G\\W]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[b-G\\0]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\t]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\n]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\v]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\f]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\r]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\c0001]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\x0061]", false), Ok(()));
        assert_ne!(validator.validate_pattern("[d-G\\u0061]", false), Ok(()));
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
