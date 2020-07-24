// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashSet;
use std::ops::{Deref, DerefMut, Range};

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
    strict: bool,
    ecma_version: EcmaVersion,
    u_flag: bool,
    n_flag: bool,
    last_int_value: usize,
    last_min_value: usize,
    last_max_value: usize,
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
            strict: false,
            ecma_version,
            u_flag: false,
            n_flag: false,
            last_int_value: 0,
            last_min_value: 0,
            last_max_value: 0,
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
        self.reset(source, 0, source.len(), u_flag);
        self.consume_pattern();

        if !self.n_flag &&
            self.ecma_version >= EcmaVersion::ES2018 &&
            self.group_names.len() > 0
         {
            self.n_flag = true;
            self.rewind(0);
            self.consume_pattern();
        }

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

    /// Validate the next characters as a RegExp `Term` production if possible.
    /// ```grammar
    /// Term[U, N]::
    ///      [strict] Assertion[+U, ?N]
    ///      [strict] Atom[+U, ?N]
    ///      [strict] Atom[+U, ?N] Quantifier
    ///      [annexB][+U] Assertion[+U, ?N]
    ///      [annexB][+U] Atom[+U, ?N]
    ///      [annexB][+U] Atom[+U, ?N] Quantifier
    ///      [annexB][~U] QuantifiableAssertion[?N] Quantifier
    ///      [annexB][~U] Assertion[~U, ?N]
    ///      [annexB][~U] ExtendedAtom[?N] Quantifier
    ///      [annexB][~U] ExtendedAtom[?N]
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    /*fn consume_term(&self) -> bool {
        if self.u_flag || self.strict {
            return
                self.consume_assertion() ||
                (self.consume_atom() && self.consume_optional_quantifier())
        }
        return
            (self.consume_assertion() &&
                (!self.last_assertion_is_quantifiable ||
                    self.consume_optional_quantifier())) ||
            (self.consume_extended_atom() && self.consume_optional_quantifier())
    }

    fn consume_optional_quantifier() -> bool {
        this.consume_quantifier()
        true
    }*/

    /// Validate the next characters as a RegExp `Quantifier` production if possible.
    /// ```grammar
    /// Quantifier::
    ///      QuantifierPrefix
    ///      QuantifierPrefix `?`
    /// QuantifierPrefix::
    ///      `*`
    ///      `+`
    ///      `?`
    ///      `{` DecimalDigits `}`
    ///      `{` DecimalDigits `,}`
    ///      `{` DecimalDigits `,` DecimalDigits `}`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_quantifier(&mut self, no_consume: bool) -> bool {
        let start = self.index();
        let mut min = 0;
        let mut max = 0;
        let mut greedy = false;

        // QuantifierPrefix
        if self.eat('*') {
            min = 0;
            max = usize::MAX;
        } else if self.eat('+') {
            min = 1;
            max = usize::MAX;
        } else if self.eat('?') {
            min = 0;
            max = 1;
        } else if self.eat_braced_quantifier(no_consume) {
            //range = self.last_min_value..self.last_max_value;
        } else {
            return false;
        }

        greedy = !self.eat('?');

        if !no_consume {
            //self.on_quantifier(start, self.index(), range, greedy);
        }
        return true;
    }

    /// Eats the next characters as the following alternatives if possible.
    /// Sets `self.last_min_value` and `self.last_max_value` if it consumed the next characters
    /// successfully.
    /// ```grammar
    ///      `{` DecimalDigits `}`
    ///      `{` DecimalDigits `,}`
    ///      `{` DecimalDigits `,` DecimalDigits `}`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn eat_braced_quantifier(&mut self, no_error: bool) -> bool {
        let start = self.index();
        if self.eat('{') {
            self.last_min_value = 0;
            self.last_max_value = usize::MAX;
            if self.eat_decimal_digits() {
                self.last_min_value = self.last_int_value;
                self.last_max_value = self.last_int_value;
                if self.eat(',') {
                    self.last_max_value = if self.eat_decimal_digits() {
                        self.last_int_value
                    } else {
                        usize::MAX
                    }
                }
                if self.eat('}') {
                    if !no_error && self.last_max_value < self.last_min_value {
                        //self.raise("numbers out of order in {} quantifier");
                    }
                    return true;
                }
            }
            if !no_error && (self.u_flag || self.strict) {
                //self.raise("Incomplete quantifier");
            }
            self.rewind(start);
        }
        return false
    }

    /// Eat the next characters as a `DecimalDigits` production if possible.
    /// Set `self.last_int_value` if it ate the next characters successfully.
    /// ```grammar
    /// DecimalDigits::
    ///      DecimalDigit
    ///      DecimalDigits DecimalDigit
    /// DecimalDigit:: one of
    ///      0 1 2 3 4 5 6 7 8 9
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_decimal_digits(&mut self) -> bool {
        let start = self.index();

        self.last_int_value = 0;
        while let Some(&c) = self.code_point_with_offset(0) {
            if !c.is_digit(10) { break; }
            self.last_int_value =
                10 * self.last_int_value +
                   self.code_point_with_offset(0).unwrap().to_digit(10).unwrap() as usize;
            self.advance();
        }

        return self.index() != start;
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

    #[test]
    fn validate_pattern_test() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        assert_eq!(validator.validate_pattern("[abc]de|fg", false), true);
    }

    #[test]
    fn count_capturing_parens_test() {
    }
}
