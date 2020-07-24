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
    num_capturing_parens: u32,
    group_names: HashSet<String>,
    backreference_names: HashSet<String>,
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
            num_capturing_parens: 0,
            group_names: HashSet::new(),
            backreference_names: HashSet::new(),
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
    fn consume_pattern(&mut self) {
        let start = self.index();
        self.num_capturing_parens = self.count_capturing_parens();
        self.group_names.clear();
        self.backreference_names.clear();

        //self.onPatternEnter(start)
        self.consume_disjunction();

        if let Some(&cp) = self.code_point_with_offset(0) {
            if cp == ')' {
                //this.raise("Unmatched ')'");
            }
            if cp == '\\' {
                //this.raise("\\ at end of pattern");
            }
            if cp == ']' || cp == '}' {
                //this.raise("Lone quantifier brackets");
            }
            //this.raise("Unexpected character {}", cp);
        }

        for name in &self.backreference_names {
            if !self.group_names.contains(name) {
                //this.raise("Invalid named capture referenced")
            }
        }
        //self.onPatternLeave(start, self.index());
    }

    /// Validate the next characters as a RegExp `Disjunction` production.
    /// ```grammar
    /// Disjunction[U, N]::
    ///      Alternative[?U, ?N]
    ///      Alternative[?U, ?N] `|` Disjunction[?U, ?N]
    /// ```
    fn consume_disjunction(&mut self) {
        let start = self.index();
        let mut i = 0;

        //self.onDisjunctionEnter(start);
        self.consume_alternative(i);
        while self.eat('|') {
            i += 1;
            self.consume_alternative(i);
        }

        //if self.consume_quantifier(true) {
            //this.raise("Nothing to repeat")
        //}
        if self.eat('{') {
            //this.raise("Lone quantifier brackets")
        }
        //self.on_disjunction_leave(start, self.index());
    }

    /// Validate the next characters as a RegExp `Alternative` production.
    /// ```grammar
    /// Alternative[U, N]::
    ///      ε
    ///      Alternative[?U, ?N] Term[?U, ?N]
    /// ```
    fn consume_alternative(&mut self, i: u32) {
        let start = self.index();

        //self.on_alternative_enter(start, i)
        //while self.code_point_with_offset(0).is_some() && self.consume_term() {
            // do nothing
        //}
        //self.on_alternative_leave(start, self.index(), i);
    }

    fn count_capturing_parens(&mut self) -> u32 {
        let start = self.index();
        let mut in_class = false;
        let mut escaped = false;
        let mut count = 0;

        while let Some(&cp) = self.code_point_with_offset(0) {
            if escaped {
                escaped = false;
            } else if cp == '\\' {
                escaped = true;
            } else if cp == '[' {
                in_class = true;
            } else if cp == ']' {
                in_class = false;
            } else if cp == '('
                && !in_class
                && (self.code_point_with_offset(1) != Some(&'?')
                    || (self.code_point_with_offset(2) == Some(&'<')
                        && self.code_point_with_offset(3) != Some(&'=')
                        && self.code_point_with_offset(3) != Some(&'!')))
            {
                count += 1
            }
            self.advance();
        }

        self.rewind(start);
        count
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
