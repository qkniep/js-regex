// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashSet;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum EcmaVersion {
    ES5,
    ES2015,
    ES2016,
    ES2017,
    ES2018,
    ES2019,
    ES2020,
    ES2021,
}

#[derive(Debug)]
pub struct EcmaRegexValidator {
    ecma_version: EcmaVersion,
}

impl EcmaRegexValidator {
    fn new(ecma_version: EcmaVersion) -> Self {
        EcmaRegexValidator {
            ecma_version
        }
    }

    fn validatePattern(&self, pattern: &str, ) -> bool {
        return true;
    }

    ///
    fn validateFlags(&self, flags: &str) -> bool {
        let mut existing_flags = HashSet::<char>::new();
        let mut global = false;
        let mut ignore_case = false;
        let mut multiline = false;
        let mut sticky = false;
        let mut unicode = false;
        let mut dot_all = false;

        for flag in flags.chars() {
            if existing_flags.contains(&flag) {
                return false;  // duplicate flag
            }
            existing_flags.insert(flag);

            if flag == 'g' {
                global = true;
            } else if flag == 'i' {
                ignore_case = true;
            } else if flag == 'm' {
                multiline = true;
            } else if flag == 'u' && self.ecma_version >= EcmaVersion::ES2015 {
                unicode = true;
            } else if flag == 'y' && self.ecma_version >= EcmaVersion::ES2015 {
                sticky = true;
            } else if flag == 's' && self.ecma_version >= EcmaVersion::ES2018 {
                dot_all = true;
            } else {
                return false;  // invalid flag
            }
        }

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_flags() {
        let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validateFlags("gimuys"), true);
        assert_eq!(validator.validateFlags("gimgu"), false);
        assert_eq!(validator.validateFlags("gimuf"), false);
    }
}
