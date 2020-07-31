// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod reader;
mod validator;

pub use validator::{EcmaRegexValidator, EcmaVersion};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_flags_test() {
        let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_flags("gimuys"), Ok(()));
        assert_eq!(validator.validate_flags("g"), Ok(()));
        assert_eq!(validator.validate_flags("gim"), Ok(()));

        assert_eq!(validator.validate_flags("gimgu"), Err("Duplicated flag g".to_string()));
        assert_eq!(validator.validate_flags("gimuf"), Err("Invalid flag f".to_string()));
        assert_eq!(validator.validate_flags("a"), Err("Invalid flag a".to_string()));
    }

    #[test]
    fn validate_pattern_test() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_pattern("[abc]de|fg", false), Ok(()));

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
}
