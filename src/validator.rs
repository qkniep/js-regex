// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use crate::reader::Reader;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
    reader: Reader,
    ecma_version: EcmaVersion,
    u_flag: bool,
    n_flag: bool,
}

impl Deref for EcmaRegexValidator {
    type Target = Reader;

    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}

impl DerefMut for EcmaRegexValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reader
    }
}

impl EcmaRegexValidator {
    fn new(ecma_version: EcmaVersion) -> Self {
        EcmaRegexValidator {
            reader: Reader::new(),
            ecma_version,
            u_flag: false,
            n_flag: false,
        }
    }

    /// Validates flags of a EcmaScript regular expression.
    pub fn validate_flags(&self, flags: &str) -> bool {
        // TODO: return Result
        let mut existing_flags = HashSet::<char>::new();
        let mut global = false;
        let mut ignore_case = false;
        let mut multiline = false;
        let mut sticky = false;
        let mut unicode = false;
        let mut dot_all = false;

        for flag in flags.chars() {
            if existing_flags.contains(&flag) {
                return false; // duplicate flag
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
                return false; // invalid flag
            }
        }

        return true;
    }

    /// Validates the pattern of a EcmaScript regular expression.
    pub fn validate_pattern(&mut self, source: &str, u_flag: bool) -> bool {
        // TODO: return Result
        self.u_flag = u_flag && self.ecma_version >= EcmaVersion::ES2015;
        self.n_flag = u_flag && self.ecma_version >= EcmaVersion::ES2018;
        //TODO: rewind
        self.consume_pattern();
        return true;
    }

    /// Validate the next characters as a RegExp `Pattern` production.
    /// ```grammar
    /// Pattern[U, N]::
    ///     Disjunction[?U, ?N]
    /// ```
    fn consume_pattern(&self) {
        let start = self.index();
        /*self.num_capturing_parens = this.countCapturingParens()
        self.group_names.clear()
        self.backreference_names.clear()

        //self.onPatternEnter(start)
        self.consumeDisjunction()

        if let Some(cp) = self.current_code_point {
            if cp == ')' {
                this.raise("Unmatched ')'");
            }
            if (cp == '\\') {
                this.raise("\\ at end of pattern");
            }
            if (cp == ']' || cp == '}') {
                this.raise("Lone quantifier brackets");
            }
            this.raise("Unexpected character {}", cp);
        }
        for name in self.backreference_names {
            if !this._groupNames.has(name) {
                this.raise("Invalid named capture referenced")
            }
        }*/
        //self.onPatternLeave(start, this.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_flags_test() {
        let validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_flags("gimuys"), true);
        assert_eq!(validator.validate_flags("gimgu"), false);
        assert_eq!(validator.validate_flags("gimuf"), false);
    }
}
