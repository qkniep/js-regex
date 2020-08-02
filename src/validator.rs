// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use crate::reader::Reader;
use crate::unicode::{is_valid_lone_unicode_property, is_valid_unicode_property};

fn is_syntax_character(cp: char) -> bool {
    return cp == '^'
        || cp == '$'
        || cp == '\\'
        || cp == '.'
        || cp == '*'
        || cp == '+'
        || cp == '?'
        || cp == '('
        || cp == ')'
        || cp == '['
        || cp == ']'
        || cp == '{'
        || cp == '}'
        || cp == '|';
}

fn is_unicode_property_name_character(cp: char) -> bool {
    cp.is_ascii_alphabetic() || cp == '_'
}

fn is_unicode_property_value_character(cp: char) -> bool {
    is_unicode_property_name_character(cp) || cp.is_digit(10)
}

fn is_regexp_identifier_start(cp: char) -> bool {
    is_id_start(cp) || cp == '$' || cp == '_'
}

fn is_regexp_identifier_part(cp: char) -> bool {
    is_id_continue(cp) ||
    cp == '$' ||
    cp == '_' ||
    cp == '\u{200c}' ||  // unicode zero-width non-joiner
    cp == '\u{200d}' // unicode zero-width joiner
}

fn is_id_start(cp: char) -> bool {
    if (cp as u32) < 0x41 {
        false
    } else if (cp as u32) < 0x5b {
        true
    } else if (cp as u32) < 0x61 {
        false
    } else if (cp as u32) < 0x7b {
        true
    } else {
        //is_large_id_start(cp)
        true
    }
}

fn is_id_continue(cp: char) -> bool {
    if (cp as u32) < 0x30 {
        false
    } else if (cp as u32) < 0x3a {
        true
    } else if (cp as u32) < 0x41 {
        false
    } else if (cp as u32) < 0x5b {
        true
    } else if (cp as u32) == 0x5f {
        true
    } else if (cp as u32) < 0x61 {
        false
    } else if (cp as u32) < 0x7b {
        true
    } else {
        //is_large_id_start(cp) || is_large_id_continue(cp)
        true
    }
}

/*fn is_large_id_start(cp: char) -> bool {
    return is_in_range(
        cp,
        largeIdStartRanges || (largeIdStartRanges = initLargeIdStartRanges()),
    )
}

fn isLargeIdContinue(cp: char) -> bool {
    return is_in_range(
        cp,
        largeIdContinueRanges ||
            (largeIdContinueRanges = initLargeIdContinueRanges()),
    )
}

fn init_large_id_start_ranges() -> &[u32] {
    return restore_ranges(
        "4q 0 b 0 5 0 6 m 2 u 2 cp 5 b f 4 8 0 2 0 3m 4 2 1 3 3 2 0 7 0 2 2 2 0 2 j 2 2a 2 3u 9 4l 2 11 3 0 7 14 20 q 5 3 1a 16 10 1 2 2q 2 0 g 1 8 1 b 2 3 0 h 0 2 t u 2g c 0 p w a 1 5 0 6 l 5 0 a 0 4 0 o o 8 a 1i k 2 h 1p 1h 4 0 j 0 8 9 g f 5 7 3 1 3 l 2 6 2 0 4 3 4 0 h 0 e 1 2 2 f 1 b 0 9 5 5 1 3 l 2 6 2 1 2 1 2 1 w 3 2 0 k 2 h 8 2 2 2 l 2 6 2 1 2 4 4 0 j 0 g 1 o 0 c 7 3 1 3 l 2 6 2 1 2 4 4 0 v 1 2 2 g 0 i 0 2 5 4 2 2 3 4 1 2 0 2 1 4 1 4 2 4 b n 0 1h 7 2 2 2 m 2 f 4 0 r 2 6 1 v 0 5 7 2 2 2 m 2 9 2 4 4 0 x 0 2 1 g 1 i 8 2 2 2 14 3 0 h 0 6 2 9 2 p 5 6 h 4 n 2 8 2 0 3 6 1n 1b 2 1 d 6 1n 1 2 0 2 4 2 n 2 0 2 9 2 1 a 0 3 4 2 0 m 3 x 0 1s 7 2 z s 4 38 16 l 0 h 5 5 3 4 0 4 1 8 2 5 c d 0 i 11 2 0 6 0 3 16 2 98 2 3 3 6 2 0 2 3 3 14 2 3 3 w 2 3 3 6 2 0 2 3 3 e 2 1k 2 3 3 1u 12 f h 2d 3 5 4 h7 3 g 2 p 6 22 4 a 8 c 2 3 f h f h f c 2 2 g 1f 10 0 5 0 1w 2g 8 14 2 0 6 1x b u 1e t 3 4 c 17 5 p 1j m a 1g 2b 0 2m 1a i 6 1k t e 1 b 17 r z 16 2 b z 3 8 8 16 3 2 16 3 2 5 2 1 4 0 6 5b 1t 7p 3 5 3 11 3 5 3 7 2 0 2 0 2 0 2 u 3 1g 2 6 2 0 4 2 2 6 4 3 3 5 5 c 6 2 2 6 39 0 e 0 h c 2u 0 5 0 3 9 2 0 3 5 7 0 2 0 2 0 2 f 3 3 6 4 5 0 i 14 22g 1a 2 1a 2 3o 7 3 4 1 d 11 2 0 6 0 3 1j 8 0 h m a 6 2 6 2 6 2 6 2 6 2 6 2 6 2 6 fb 2 q 8 8 4 3 4 5 2d 5 4 2 2h 2 3 6 16 2 2l i v 1d f e9 533 1t g70 4 wc 1w 19 3 7g 4 f b 1 l 1a h u 3 27 14 8 3 2u 3 1g 3 8 17 c 2 2 2 3 2 m u 1f f 1d 1r 5 4 0 2 1 c r b m q s 8 1a t 0 h 4 2 9 b 4 2 14 o 2 2 7 l m 4 0 4 1d 2 0 4 1 3 4 3 0 2 0 p 2 3 a 8 2 d 5 3 5 3 5 a 6 2 6 2 16 2 d 7 36 u 8mb d m 5 1c 6it a5 3 2x 13 6 d 4 6 0 2 9 2 c 2 4 2 0 2 1 2 1 2 2z y a2 j 1r 3 1h 15 b 39 4 2 3q 11 p 7 p c 2g 4 5 3 5 3 5 3 2 10 b 2 p 2 i 2 1 2 e 3 d z 3e 1y 1g 7g s 4 1c 1c v e t 6 11 b t 3 z 5 7 2 4 17 4d j z 5 z 5 13 9 1f 4d 8m a l b 7 49 5 3 0 2 17 2 1 4 0 3 m b m a u 1u i 2 1 b l b p 1z 1j 7 1 1t 0 g 3 2 2 2 s 17 s 4 s 10 7 2 r s 1h b l b i e h 33 20 1k 1e e 1e e z 9p 15 7 1 27 s b 0 9 l 2z k s m d 1g 24 18 x o r z u 0 3 0 9 y 4 0 d 1b f 3 m 0 2 0 10 h 2 o 2d 6 2 0 2 3 2 e 2 9 8 1a 13 7 3 1 3 l 2 6 2 1 2 4 4 0 j 0 d 4 4f 1g j 3 l 2 v 1b l 1 2 0 55 1a 16 3 11 1b l 0 1o 16 e 0 20 q 6e 17 39 1r w 7 3 0 3 7 2 1 2 n g 0 2 0 2n 7 3 12 h 0 2 0 t 0 b 13 8 0 m 0 c 19 k 0 z 1k 7c 8 2 10 i 0 1e t 35 6 2 1 2 11 m 0 q 5 2 1 2 v f 0 94 i 5a 0 28 pl 2v 32 i 5f 24d tq 34i g6 6nu fs 8 u 36 t j 1b h 3 w k 6 i j5 1r 3l 22 6 0 1v c 1t 1 2 0 t 4qf 9 yd 17 8 6wo 7y 1e 2 i 3 9 az 1s5 2y 6 c 4 8 8 9 4mf 2c 2 1y 2 1 3 0 3 1 3 3 2 b 2 0 2 6 2 1s 2 3 3 7 2 6 2 r 2 3 2 4 2 0 4 6 2 9f 3 o 2 o 2 u 2 o 2 u 2 o 2 u 2 o 2 u 2 o 2 7 1th 18 b 6 h 0 aa 17 105 5g 1o 1v 8 0 xh 3 2 q 2 1 2 0 3 0 2 9 2 3 2 0 2 0 7 0 5 0 2 0 2 0 2 2 2 1 2 0 3 0 2 0 2 0 2 0 2 0 2 1 2 0 3 3 2 6 2 3 2 3 2 0 2 9 2 g 6 2 2 4 2 g 3et wyl z 378 c 65 3 4g1 f 5rk 2e8 f1 15v 3t6",
    )
}

fn init_large_id_continue_ranges() -> &[u32] {
    return restoreRanges(
        "53 0 g9 33 o 0 70 4 7e 18 2 0 2 1 2 1 2 0 21 a 1d u 7 0 2u 6 3 5 3 1 2 3 3 9 o 0 v q 2k a g 9 y 8 a 0 p 3 2 8 2 2 2 4 18 2 3c e 2 w 1j 2 2 h 2 6 b 1 3 9 i 2 1l 0 2 6 3 1 3 2 a 0 b 1 3 9 f 0 3 2 1l 0 2 4 5 1 3 2 4 0 l b 4 0 c 2 1l 0 2 7 2 2 2 2 l 1 3 9 b 5 2 2 1l 0 2 6 3 1 3 2 8 2 b 1 3 9 j 0 1o 4 4 2 2 3 a 0 f 9 h 4 1m 6 2 2 2 3 8 1 c 1 3 9 i 2 1l 0 2 6 2 2 2 3 8 1 c 1 3 9 h 3 1k 1 2 6 2 2 2 3 a 0 b 1 3 9 i 2 1z 0 5 5 2 0 2 7 7 9 3 1 1q 0 3 6 d 7 2 9 2g 0 3 8 c 5 3 9 1r 1 7 9 c 0 2 0 2 0 5 1 1e j 2 1 6 a 2 z a 0 2t j 2 9 d 3 5 2 2 2 3 6 4 3 e b 2 e jk 2 a 8 pt 2 u 2 u 1 v 1 1t v a 0 3 9 y 2 3 9 40 0 3b b 5 b b 9 3l a 1p 4 1m 9 2 s 3 a 7 9 n d 2 1 1s 4 1c g c 9 i 8 d 2 v c 3 9 19 d 1d j 9 9 7 9 3b 2 2 k 5 0 7 0 3 2 5j 1l 2 4 g0 1 k 0 3g c 5 0 4 b 2db 2 3y 0 2p v ff 5 2y 1 n7q 9 1y 0 5 9 x 1 29 1 7l 0 4 0 5 0 o 4 5 0 2c 1 1f h b 9 7 h e a t 7 q c 19 3 1c d g 9 c 0 b 9 1c d d 0 9 1 3 9 y 2 1f 0 2 2 3 1 6 1 2 0 16 4 6 1 6l 7 2 1 3 9 fmt 0 ki f h f 4 1 p 2 5d 9 12 0 ji 0 6b 0 46 4 86 9 120 2 2 1 6 3 15 2 5 0 4m 1 fy 3 9 9 aa 1 4a a 4w 2 1i e w 9 g 3 1a a 1i 9 7 2 11 d 2 9 6 1 19 0 d 2 1d d 9 3 2 b 2b b 7 0 4h b 6 9 7 3 1k 1 2 6 3 1 3 2 a 0 b 1 3 6 4 4 5d h a 9 5 0 2a j d 9 5y 6 3 8 s 1 2b g g 9 2a c 9 9 2c e 5 9 6r e 4m 9 1z 5 2 1 3 3 2 0 2 1 d 9 3c 6 3 6 4 0 t 9 15 6 2 3 9 0 a a 1b f ba 7 2 7 h 9 1l l 2 d 3f 5 4 0 2 1 2 6 2 0 9 9 1d 4 2 1 2 4 9 9 96 3 ewa 9 3r 4 1o 6 q 9 s6 0 2 1i 8 3 2a 0 c 1 f58 1 43r 4 4 5 9 7 3 6 v 3 45 2 13e 1d e9 1i 5 1d 9 0 f 0 n 4 2 e 11t 6 2 g 3 6 2 1 2 4 7a 6 a 9 bn d 15j 6 32 6 6 9 3o7 9 gvt3 6n",
    )
}

function isInRange(cp: number, ranges: number[]): boolean {
    let l = 0,
        r = (ranges.length / 2) | 0,
        i = 0,
        min = 0,
        max = 0
    while (l < r) {
        i = ((l + r) / 2) | 0
        min = ranges[2 * i]
        max = ranges[2 * i + 1]
        if (cp < min) {
            r = i
        } else if (cp > max) {
            l = i + 1
        } else {
            return true
        }
    }
    return false
}

fn restore_ranges(data: &str) -> number[] {
    let last = 0
    return data.split(" ").map(s => (last += parseInt(s, 36) | 0))
}*/

fn is_valid_unicode(cp: usize) -> bool {
    cp <= 0x10ffff
}

fn is_lead_surrogate(cp: usize) -> bool {
    cp >= 0xd800 && cp <= 0xdbff
}

fn is_trail_surrogate(cp: usize) -> bool {
    cp >= 0xdc00 && cp <= 0xdfff
}

fn combine_surrogate_pair(lead: usize, trail: usize) -> usize {
    (lead - 0xd800) * 0x400 + (trail - 0xdc00) + 0x10000
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EcmaVersion {
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
    last_str_value: String,
    last_key_value: String,
    last_val_value: String,
    last_assertion_is_quantifiable: bool,
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
    pub fn new(ecma_version: EcmaVersion) -> Self {
        EcmaRegexValidator {
            reader: Reader::new(),
            strict: false,
            ecma_version,
            u_flag: false,
            n_flag: false,
            last_int_value: 0,
            last_min_value: 0,
            last_max_value: 0,
            last_str_value: "".to_string(),
            last_key_value: "".to_string(),
            last_val_value: "".to_string(),
            last_assertion_is_quantifiable: false,
            num_capturing_parens: 0,
            group_names: HashSet::new(),
            backreference_names: HashSet::new(),
        }
    }

    /// Validates flags of a EcmaScript regular expression.
    pub fn validate_flags(&self, flags: &str) -> Result<(), String> {
        let mut existing_flags = HashSet::<char>::new();

        for flag in flags.chars() {
            if existing_flags.contains(&flag) {
                return Err(format!("Duplicated flag {}", flag));
            }
            existing_flags.insert(flag);

            if flag == 'g'
                || flag == 'i'
                || flag == 'm'
                || (flag == 'u' && self.ecma_version >= EcmaVersion::ES2015)
                || (flag == 'y' && self.ecma_version >= EcmaVersion::ES2015)
                || (flag == 's' && self.ecma_version >= EcmaVersion::ES2018)
            {
                // do nothing
            } else {
                return Err(format!("Invalid flag {}", flag));
            }
        }
        Ok(())
    }

    /// Validates the pattern of a EcmaScript regular expression.
    pub fn validate_pattern(&mut self, source: &str, u_flag: bool) -> Result<(), String> {
        self.u_flag = u_flag && self.ecma_version >= EcmaVersion::ES2015;
        self.n_flag = u_flag && self.ecma_version >= EcmaVersion::ES2018;
        //self.reset(source, 0, source.len(), u_flag);
        self.reset(source, 0, source.chars().count(), u_flag);
        self.consume_pattern()?;

        if !self.n_flag && self.ecma_version >= EcmaVersion::ES2018 && self.group_names.len() > 0 {
            self.n_flag = true;
            self.rewind(0);
            self.consume_pattern()?;
        }

        return Ok(());
    }

    /// Validate the next characters as a RegExp `Pattern` production.
    /// ```grammar
    /// Pattern[U, N]::
    ///     Disjunction[?U, ?N]
    /// ```
    fn consume_pattern(&mut self) -> Result<(), String> {
        self.num_capturing_parens = self.count_capturing_parens();
        self.group_names.clear();
        self.backreference_names.clear();

        self.consume_disjunction()?;

        if let Some(cp) = self.code_point_with_offset(0) {
            if cp == ')' {
                return Err("Unmatched ')'".to_string());
            } else if cp == '\\' {
                return Err("\\ at end of pattern".to_string());
            } else if cp == ']' || cp == '}' {
                return Err("Lone quantifier brackets".to_string());
            }
            return Err(format!("Unexpected character {}", cp));
        }

        for name in self.backreference_names.difference(&self.group_names) {
            return Err(format!("Invalid named capture referenced: {}", name));
        }
        return Ok(());
    }

    /// Validate the next characters as a RegExp `Disjunction` production.
    /// ```grammar
    /// Disjunction[U, N]::
    ///      Alternative[?U, ?N]
    ///      Alternative[?U, ?N] `|` Disjunction[?U, ?N]
    /// ```
    fn consume_disjunction(&mut self) -> Result<(), String> {
        self.consume_alternative()?;
        while self.eat('|') {
            self.consume_alternative()?;
        }

        if self.consume_quantifier(true)? {
            return Err("Nothing to repeat".to_string());
        } else if self.eat('{') {
            return Err("Lone quantifier brackets".to_string());
        }
        return Ok(());
    }

    /// Validate the next characters as a RegExp `Alternative` production.
    /// ```grammar
    /// Alternative[U, N]::
    ///      ε
    ///      Alternative[?U, ?N] Term[?U, ?N]
    /// ```
    fn consume_alternative(&mut self) -> Result<(), String> {
        while self.code_point_with_offset(0).is_some() && self.consume_term()? {
            // do nothing
        }
        Ok(())
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
    fn consume_term(&mut self) -> Result<bool, String> {
        if self.u_flag || self.strict {
            return Ok(self.consume_assertion()?
                || (self.consume_atom()? && self.consume_optional_quantifier()?));
        }
        return Ok((self.consume_assertion()?
            && (!self.last_assertion_is_quantifiable || self.consume_optional_quantifier()?))
            || (self.consume_extended_atom()? && self.consume_optional_quantifier()?));
    }

    fn consume_optional_quantifier(&mut self) -> Result<bool, String> {
        self.consume_quantifier(false)?;
        Ok(true)
    }

    /// Validate the next characters as a RegExp `Term` production if possible.
    /// Set `self.last_assertion_is_quantifiable` if the consumed assertion was a
    /// `QuantifiableAssertion` production.
    /// ```grammar
    /// Assertion[U, N]::
    ///      `^`
    ///      `$`
    ///      `\b`
    ///      `\B`
    ///      [strict] `(?=` Disjunction[+U, ?N] `)`
    ///      [strict] `(?!` Disjunction[+U, ?N] `)`
    ///      [annexB][+U] `(?=` Disjunction[+U, ?N] `)`
    ///      [annexB][+U] `(?!` Disjunction[+U, ?N] `)`
    ///      [annexB][~U] QuantifiableAssertion[?N]
    ///      `(?<=` Disjunction[?U, ?N] `)`
    ///      `(?<!` Disjunction[?U, ?N] `)`
    /// QuantifiableAssertion[N]::
    ///      `(?=` Disjunction[~U, ?N] `)`
    ///      `(?!` Disjunction[~U, ?N] `)`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_assertion(&mut self) -> Result<bool, String> {
        let start = self.index();
        self.last_assertion_is_quantifiable = false;

        if self.eat('^') || self.eat('$') || self.eat2('\\', 'B') || self.eat2('\\', 'b') {
            return Ok(true);
        }

        // Lookahead / Lookbehind
        if self.eat2('(', '?') {
            let lookbehind = self.ecma_version >= EcmaVersion::ES2018 && self.eat('<');
            let mut flag = self.eat('=');
            if !flag {
                flag = self.eat('!');
            }
            if flag {
                self.consume_disjunction()?;
                if !self.eat(')') {
                    return Err("Unterminated group".to_string());
                }
                self.last_assertion_is_quantifiable = !lookbehind && !self.strict;
                return Ok(true);
            }
            self.rewind(start);
        }
        Ok(false)
    }

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
    fn consume_quantifier(&mut self, no_consume: bool) -> Result<bool, String> {
        // QuantifierPrefix
        if !self.eat('*')
            && !self.eat('+')
            && !self.eat('?')
            && !self.eat_braced_quantifier(no_consume)?
        {
            return Ok(false);
        }

        self.eat('?');
        return Ok(true);
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
    fn eat_braced_quantifier(&mut self, no_error: bool) -> Result<bool, &str> {
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
                        return Err("numbers out of order in {} quantifier");
                    }
                    return Ok(true);
                }
            }
            if !no_error && (self.u_flag || self.strict) {
                return Err("Incomplete quantifier");
            }
            self.rewind(start);
        }
        return Ok(false);
    }

    /// Validate the next characters as a RegExp `Atom` production if possible.
    /// ```grammar
    /// Atom[U, N]::
    ///      PatternCharacter
    ///      `.`
    ///      `\\` AtomEscape[?U, ?N]
    ///      CharacterClass[?U]
    ///      `(?:` Disjunction[?U, ?N] )
    ///      `(` GroupSpecifier[?U] Disjunction[?U, ?N] `)`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_atom(&mut self) -> Result<bool, String> {
        Ok(self.consume_pattern_character()
            || self.consume_dot()
            || self.consume_reverse_solidus_atom_escape()?
            || self.consume_character_class()?
            || self.consume_uncapturing_group()?
            || self.consume_capturing_group()?)
    }

    /// Validate the next characters as the following alternatives if possible.
    /// ```grammar
    ///      `.`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_dot(&mut self) -> bool {
        if self.eat('.') {
            return true;
        }
        return false;
    }

    /// Validate the next characters as the following alternatives if possible.
    /// ```grammar
    ///      `\\` AtomEscape[?U, ?N]
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_reverse_solidus_atom_escape(&mut self) -> Result<bool, String> {
        let start = self.index();
        if self.eat('\\') {
            if self.consume_atom_escape()? {
                return Ok(true);
            }
            self.rewind(start);
        }
        return Ok(false);
    }

    /// Validate the next characters as the following alternatives if possible.
    /// ```grammar
    ///      `(?:` Disjunction[?U, ?N] )
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_uncapturing_group(&mut self) -> Result<bool, String> {
        if self.eat3('(', '?', ':') {
            self.consume_disjunction()?;
            if !self.eat(')') {
                return Err("Unterminated group".to_string());
            }
            return Ok(true);
        }
        return Ok(false);
    }

    /// Validate the next characters as the following alternatives if possible.
    /// ```grammar
    ///      `(` GroupSpecifier[?U] Disjunction[?U, ?N] `)`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_capturing_group(&mut self) -> Result<bool, String> {
        if !self.eat('(') {
            return Ok(false);
        }

        if self.ecma_version >= EcmaVersion::ES2018 {
            self.consume_group_specifier()?;
        } else if self.code_point_with_offset(0) == Some('?') {
            return Err("Invalid group".to_string());
        }

        self.consume_disjunction()?;
        if !self.eat(')') {
            return Err("Unterminated group".to_string());
        }
        Ok(true)
    }

    /// Validate the next characters as a RegExp `ExtendedAtom` production if possible.
    /// ```grammar
    /// ExtendedAtom[N]::
    ///      `.`
    ///      `\` AtomEscape[~U, ?N]
    ///      `\` [lookahead = c]
    ///      CharacterClass[~U]
    ///      `(?:` Disjunction[~U, ?N] `)`
    ///      `(` Disjunction[~U, ?N] `)`
    ///      InvalidBracedQuantifier
    ///      ExtendedPatternCharacter
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_extended_atom(&mut self) -> Result<bool, String> {
        Ok(self.eat('.')
            || self.consume_reverse_solidus_atom_escape()?
            || self.consume_reverse_solidus_followed_by_c()
            || self.consume_character_class()?
            || self.consume_uncapturing_group()?
            || self.consume_capturing_group()?
            || self.consume_invalid_braced_quantifier()?
            || self.consume_extended_pattern_character())
    }

    /// Validate the next characters as the following alternatives if possible.
    /// ```grammar
    ///      `\` [lookahead = c]
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_reverse_solidus_followed_by_c(&mut self) -> bool {
        if self.code_point_with_offset(0) == Some('\\')
            && self.code_point_with_offset(1) == Some('c')
        {
            self.last_int_value = '\\' as usize;
            self.advance();
            return true;
        }
        return false;
    }

    /// Validate the next characters as a RegExp `InvalidBracedQuantifier`
    /// production if possible.
    /// ```grammar
    /// InvalidBracedQuantifier::
    ///      `{` DecimalDigits `}`
    ///      `{` DecimalDigits `,}`
    ///      `{` DecimalDigits `,` DecimalDigits `}`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_invalid_braced_quantifier(&mut self) -> Result<bool, &str> {
        if self.eat_braced_quantifier(true)? {
            return Err("Nothing to repeat");
        }
        Ok(false)
    }

    /// Validate the next characters as a RegExp `PatternCharacter` production if
    /// possible.
    /// ```grammar
    /// PatternCharacter::
    ///      SourceCharacter but not SyntaxCharacter
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_pattern_character(&mut self) -> bool {
        if let Some(cp) = self.code_point_with_offset(0) {
            if !is_syntax_character(cp) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    /// Validate the next characters as a RegExp `ExtendedPatternCharacter`
    /// production if possible.
    /// ```grammar
    /// ExtendedPatternCharacter::
    ///      SourceCharacter but not one of ^ $ \ . * + ? ( ) [ |
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_extended_pattern_character(&mut self) -> bool {
        if let Some(cp) = self.code_point_with_offset(0) {
            if cp != '^'
                && cp != '$'
                && cp != '\\'
                && cp != '.'
                && cp != '*'
                && cp != '+'
                && cp != '?'
                && cp != '('
                && cp != ')'
                && cp != '['
                && cp != '|'
            {
                self.advance();
                return true;
            }
        }
        return false;
    }

    /// Validate the next characters as a RegExp `GroupSpecifier` production.
    /// Set `self.last_str_value` if the group name existed.
    /// ```grammar
    /// GroupSpecifier[U]::
    ///      ε
    ///      `?` GroupName[?U]
    /// ```
    /// Returns `true` if the group name existed.
    fn consume_group_specifier(&mut self) -> Result<bool, String> {
        if self.eat('?') {
            if self.eat_group_name()? {
                if !self.group_names.contains(&self.last_str_value) {
                    self.group_names.insert(self.last_str_value.clone());
                    return Ok(true);
                }
                return Err("Duplicate capture group name".to_string());
            }
            return Err("Invalid group".to_string());
        }
        return Ok(false);
    }

    /// Validate the next characters as a RegExp `AtomEscape` production if possible.
    /// ```grammar
    /// AtomEscape[U, N]::
    ///      [strict] DecimalEscape
    ///      [annexB][+U] DecimalEscape
    ///      [annexB][~U] DecimalEscape but only if the CapturingGroupNumber of DecimalEscape is <= NcapturingParens
    ///      CharacterClassEscape[?U]
    ///      [strict] CharacterEscape[?U]
    ///      [annexB] CharacterEscape[?U, ?N]
    ///      [+N] `k` GroupName[?U]
    /// ```
    /// Returns `Ok(true)` if it consumed the next characters successfully.
    fn consume_atom_escape(&mut self) -> Result<bool, String> {
        if self.consume_backreference()?
            || self.consume_character_class_escape()?
            || self.consume_character_escape()?
            || (self.n_flag && self.consume_k_group_name()?)
        {
            return Ok(true);
        }
        if self.strict || self.u_flag {
            return Err("Invalid escape".to_string());
        }
        return Ok(false);
    }

    /// Validate the next characters as the follwoing alternatives if possible.
    /// ```grammar
    ///      [strict] DecimalEscape
    ///      [annexB][+U] DecimalEscape
    ///      [annexB][~U] DecimalEscape but only if the CapturingGroupNumber of DecimalEscape is <= NcapturingParens
    /// ```
    /// Returns `Ok(true)` if it consumed the next characters successfully.
    fn consume_backreference(&mut self) -> Result<bool, &str> {
        let start = self.index();
        if self.eat_decimal_escape() {
            if self.last_int_value <= self.num_capturing_parens as usize {
                return Ok(true);
            } else if self.strict || self.u_flag {
                return Err("Invalid escape");
            }
            self.rewind(start);
        }
        Ok(false)
    }

    /// Validate the next characters as a RegExp `DecimalEscape` production if possible.
    /// Set `-1` to `self.last_int_value` as meaning of a character set if it ate the next
    /// characters successfully.
    /// ```grammar
    /// CharacterClassEscape[U]::
    ///      `d`
    ///      `D`
    ///      `s`
    ///      `S`
    ///      `w`
    ///      `W`
    ///      [+U] `p{` UnicodePropertyValueExpression `}`
    ///      [+U] `P{` UnicodePropertyValueExpression `}`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_character_class_escape(&mut self) -> Result<bool, String> {
        let start = self.index();

        if self.eat('d')
            || self.eat('D')
            || self.eat('s')
            || self.eat('S')
            || self.eat('w')
            || self.eat('W')
        {
            //self.last_int_value = -1;
            return Ok(true);
        }

        if self.u_flag
            && self.ecma_version >= EcmaVersion::ES2018
            && (self.eat('p') || self.eat('P'))
        {
            //self.last_int_value = -1;
            if self.eat('{') && self.eat_unicode_property_value_expression()? && self.eat('}') {
                return Ok(true);
            }
            return Err("Invalid property name".to_string());
        }
        Ok(false)
    }

    /// Validate the next characters as a RegExp `CharacterEscape` production if possible.
    /// ```grammar
    /// CharacterEscape[U, N]::
    ///      ControlEscape
    ///      `c` ControlLetter
    ///      `0` [lookahead ∉ DecimalDigit]
    ///      HexEscapeSequence
    ///      RegExpUnicodeEscapeSequence[?U]
    ///      [annexB][~U] LegacyOctalEscapeSequence
    ///      IdentityEscape[?U, ?N]
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_character_escape(&mut self) -> Result<bool, String> {
        Ok(self.eat_control_escape()
            || self.eat_c_control_letter()
            || self.eat_zero()
            || self.eat_hex_escape_sequence()?
            || self.eat_regexp_unicode_escape_sequence(false)?
            || (!self.strict && !self.u_flag && self.eat_legacy_octal_escape_sequence())
            || self.eat_identity_escape())
    }

    /// Validate the next characters as the follwoing alternatives if possible.
    /// ```grammar
    ///      `k` GroupName[?U]
    /// ```
    /// Returns `Ok(true)` if it consumed the next characters successfully.
    fn consume_k_group_name(&mut self) -> Result<bool, String> {
        if self.eat('k') {
            if self.eat_group_name()? {
                let group_name = self.last_str_value.clone();
                self.backreference_names.insert(group_name);
                return Ok(true);
            }
            return Err("Invalid named reference".to_string());
        }
        Ok(false)
    }

    /// Validate the next characters as a RegExp `CharacterClass` production if possible.
    /// ```grammar
    /// CharacterClass[U]::
    ///      `[` [lookahead ≠ ^] ClassRanges[?U] `]`
    ///      `[^` ClassRanges[?U] `]`
    /// ```
    /// Returns `true` if it consumed the next characters successfully.
    fn consume_character_class(&mut self) -> Result<bool, String> {
        if !self.eat('[') {
            return Ok(false);
        }
        self.consume_class_ranges()?;
        if !self.eat(']') {
            return Err("Unterminated character class".to_string());
        }
        Ok(true)
    }

    /// Validate the next characters as a RegExp `ClassRanges` production.
    /// ```grammar
    /// ClassRanges[U]::
    ///      ε
    ///      NonemptyClassRanges[?U]
    /// NonemptyClassRanges[U]::
    ///      ClassAtom[?U]
    ///      ClassAtom[?U] NonemptyClassRangesNoDash[?U]
    ///      ClassAtom[?U] `-` ClassAtom[?U] ClassRanges[?U]
    /// NonemptyClassRangesNoDash[U]::
    ///      ClassAtom[?U]
    ///      ClassAtomNoDash[?U] NonemptyClassRangesNoDash[?U]
    ///      ClassAtomNoDash[?U] `-` ClassAtom[?U] ClassRanges[?U]
    /// ```
    fn consume_class_ranges(&mut self) -> Result<(), String> {
        let strict = self.strict || self.u_flag;
        loop {
            // Consume the first ClassAtom
            if !self.consume_class_atom()? {
                break;
            }
            let min = self.last_int_value;

            // Consume `-`
            if !self.eat('-') {
                continue;
            }

            // Consume the second ClassAtom
            if !self.consume_class_atom()? {
                break;
            }
            let max = self.last_int_value;

            // Validate
            /*if min == -1 || max == -1 {
                if strict {
                    return Err("Invalid character class".to_string());
                }
                continue
            }*/
            if min > max {
                return Err("Range out of order in character class".to_string());
            }
        }
        Ok(())
    }

    /// Validate the next characters as a RegExp `ClassAtom` production if possible.
    /// Set `self.last_int_value` if it consumed the next characters successfully.
    /// ```grammar
    /// ClassAtom[U, N]::
    ///      `-`
    ///      ClassAtomNoDash[?U, ?N]
    /// ClassAtomNoDash[U, N]::
    ///      SourceCharacter but not one of \ ] -
    ///      `\` ClassEscape[?U, ?N]
    ///      [annexB] `\` [lookahead = c]
    /// ```
    /// Returns `Ok(true)` if it consumed the next characters successfully.
    fn consume_class_atom(&mut self) -> Result<bool, String> {
        let start = self.index();

        if let Some(cp) = self.code_point_with_offset(0) {
            if cp != '\\' && cp != ']' {
                self.advance();
                self.last_int_value = cp as usize;
                return Ok(true);
            }
        }

        if self.eat('\\') {
            if self.consume_class_escape()? {
                return Ok(true);
            }
            if !self.strict && self.code_point_with_offset(0) == Some('c') {
                self.last_int_value = '\\' as usize;
                return Ok(true);
            }
            if self.strict || self.u_flag {
                return Err("Invalid escape".to_string());
            }
            self.rewind(start);
        }
        Ok(false)
    }

    /// Validate the next characters as a RegExp `ClassEscape` production if possible.
    /// Set `self.last_int_value` if it consumed the next characters successfully.
    /// ```grammar
    /// ClassEscape[U, N]::
    ///      `b`
    ///      [+U] `-`
    ///      [annexB][~U] `c` ClassControlLetter
    ///      CharacterClassEscape[?U]
    ///      CharacterEscape[?U, ?N]
    /// ClassControlLetter::
    ///      DecimalDigit
    ///      `_`
    /// ```
    /// Returns `Ok(true)` if it consumed the next characters successfully.
    fn consume_class_escape(&mut self) -> Result<bool, String> {
        if self.eat('b') {
            self.last_int_value = 0x7f; // backspace
            return Ok(true);
        }

        // [+U] `-`
        if self.u_flag && self.eat('-') {
            self.last_int_value = '-' as usize;
            return Ok(true);
        }

        // [annexB][~U] `c` ClassControlLetter
        if !self.strict && !self.u_flag && self.code_point_with_offset(0) == Some('c') {
            if let Some(cp) = self.code_point_with_offset(1) {
                if cp.is_digit(10) || cp == '_' {
                    self.advance();
                    self.advance();
                    self.last_int_value = cp as usize % 0x20;
                    return Ok(true);
                }
            }
        }

        Ok(self.consume_character_class_escape()? || self.consume_character_escape()?)
    }

    /// Eat the next characters as a RegExp `GroupName` production if possible.
    /// Set `self.last_str_value` if the group name existed.
    /// ```grammar
    /// GroupName[U]::
    ///      `<` RegExpIdentifierName[?U] `>`
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_group_name(&mut self) -> Result<bool, String> {
        if self.eat('<') {
            if self.eat_regexp_identifier_name()? && self.eat('>') {
                return Ok(true);
            }
            return Err("Invalid capture group name".to_string());
        }
        return Ok(false);
    }

    /// Eat the next characters as a RegExp `RegExpIdentifierName` production if
    /// possible.
    /// Set `self.last_str_value` if the identifier name existed.
    /// ```grammar
    /// RegExpIdentifierName[U]::
    ///      RegExpIdentifierStart[?U]
    ///      RegExpIdentifierName[?U] RegExpIdentifierPart[?U]
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_regexp_identifier_name(&mut self) -> Result<bool, String> {
        if self.eat_regexp_identifier_start()? {
            // TODO: maybe use u32 for self.last_int_value?
            self.last_str_value = std::char::from_u32(self.last_int_value as u32)
                .unwrap()
                .to_string();
            while self.eat_regexp_identifier_part()? {
                self.last_str_value
                    .push(std::char::from_u32(self.last_int_value as u32).unwrap());
            }
            return Ok(true);
        }
        return Ok(false);
    }

    /// Eat the next characters as a RegExp `RegExpIdentifierStart` production if
    /// possible.
    /// Set `self.last_int_value` if the identifier start existed.
    /// ```grammar
    /// RegExpIdentifierStart[U] ::
    ///      UnicodeIDStart
    ///      `$`
    ///      `_`
    ///      `\` RegExpUnicodeEscapeSequence[+U]
    ///      [~U] UnicodeLeadSurrogate UnicodeTrailSurrogate
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_regexp_identifier_start(&mut self) -> Result<bool, String> {
        let start = self.index();
        let force_u_flag = !self.u_flag && self.ecma_version >= EcmaVersion::ES2020;

        if let Some(mut cp) = self.code_point_with_offset(0) {
            self.advance();
            let cp1 = self.code_point_with_offset(0);
            if cp == '\\' && self.eat_regexp_unicode_escape_sequence(force_u_flag)? {
                cp = std::char::from_u32(self.last_int_value as u32).unwrap();
            } else if force_u_flag
                && is_lead_surrogate(cp as usize)
                && cp1.is_some()
                && is_trail_surrogate(cp1.unwrap() as usize)
            {
                cp = std::char::from_u32(
                    combine_surrogate_pair(cp as usize, cp1.unwrap() as usize) as u32,
                )
                .unwrap();
                self.advance();
            }

            if is_regexp_identifier_start(cp) {
                self.last_int_value = cp as usize;
                return Ok(true);
            }
        }

        if self.index() != start {
            self.rewind(start);
        }
        return Ok(false);
    }

    /// Eat the next characters as a RegExp `RegExpIdentifierPart` production if
    /// possible.
    /// Set `self.last_int_value` if the identifier part existed.
    /// ```grammar
    /// RegExpIdentifierPart[U] ::
    ///      UnicodeIDContinue
    ///      `$`
    ///      `_`
    ///      `\` RegExpUnicodeEscapeSequence[+U]
    ///      [~U] UnicodeLeadSurrogate UnicodeTrailSurrogate
    ///      <ZWNJ>
    ///      <ZWJ>
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_regexp_identifier_part(&mut self) -> Result<bool, String> {
        let start = self.index();
        let force_u_flag = !self.u_flag && self.ecma_version >= EcmaVersion::ES2020;
        let mut cp = self.code_point_with_offset(0);
        self.advance();
        let cp1 = self.code_point_with_offset(0);

        if cp == Some('\\') && self.eat_regexp_unicode_escape_sequence(force_u_flag)? {
            // TODO: convert unicode code point to char
            cp = std::char::from_u32(self.last_int_value as u32);
        } else if force_u_flag
            && is_lead_surrogate(cp.unwrap() as usize)
            && is_trail_surrogate(cp1.unwrap() as usize)
        {
            // TODO: combine UTF-16 unicode surrogates into one char
            cp = std::char::from_u32(combine_surrogate_pair(
                cp.unwrap() as usize,
                cp1.unwrap() as usize,
            ) as u32);
            self.advance();
        }

        if is_regexp_identifier_part(cp.unwrap()) {
            self.last_int_value = cp.unwrap() as usize;
            return Ok(true);
        }

        if self.index() != start {
            self.rewind(start);
        }
        return Ok(false);
    }

    /// Eat the next characters as the follwoing alternatives if possible.
    /// Set `self.last_int_value` if it ate the next characters successfully.
    /// ```grammar
    ///      `c` ControlLetter
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_c_control_letter(&mut self) -> bool {
        let start = self.index();
        if self.eat('c') {
            if self.eat_control_letter() {
                return true;
            }
            self.rewind(start);
        }
        return false;
    }

    /// Eat the next characters as the follwoing alternatives if possible.
    /// Set `self.last_int_value` if it ate the next characters successfully.
    /// ```grammar
    ///      `0` [lookahead ∉ DecimalDigit]
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_zero(&mut self) -> bool {
        if self.code_point_with_offset(0) != Some('0') {
            return false;
        }
        if let Some(cp) = self.code_point_with_offset(1) {
            if cp.is_digit(10) {
                return false;
            }
        }
        self.last_int_value = 0;
        self.advance();
        return true;
    }

    /// Eat the next characters as a RegExp `ControlEscape` production if
    /// possible.
    /// Set `self.last_int_value` if it ate the next characters successfully.
    /// ```grammar
    /// ControlEscape:: one of
    ///      f n r t v
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_control_escape(&mut self) -> bool {
        if self.eat('f') {
            //this._lastIntValue = FormFeed
            return true;
        }
        if self.eat('n') {
            //this._lastIntValue = LineFeed
            return true;
        }
        if self.eat('r') {
            //this._lastIntValue = CarriageReturn
            return true;
        }
        if self.eat('t') {
            //this._lastIntValue = CharacterTabulation
            return true;
        }
        if self.eat('v') {
            //this._lastIntValue = LineTabulation
            return true;
        }
        return false;
    }

    /// Eat the next characters as a RegExp `ControlLetter` production if
    /// possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    /// ControlLetter:: one of
    ///      a b c d e f g h i j k l m n o p q r s t u v w x y z
    ///      A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_control_letter(&mut self) -> bool {
        if let Some(cp) = self.code_point_with_offset(0) {
            if cp.is_ascii_alphabetic() {
                self.advance();
                self.last_int_value = cp as usize % 0x20;
                return true;
            }
        }
        return false;
    }

    /// Eat the next characters as a RegExp `RegExpUnicodeEscapeSequence`
    /// production if possible.
    /// Set `self.last_int_value` if it ate the next characters successfully.
    /// ```grammar
    /// RegExpUnicodeEscapeSequence[U]::
    ///      [+U] `u` LeadSurrogate `\u` TrailSurrogate
    ///      [+U] `u` LeadSurrogate
    ///      [+U] `u` TrailSurrogate
    ///      [+U] `u` NonSurrogate
    ///      [~U] `u` Hex4Digits
    ///      [+U] `u{` CodePoint `}`
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_regexp_unicode_escape_sequence(&mut self, force_u_flag: bool) -> Result<bool, &str> {
        let start = self.index();
        let u_flag = force_u_flag || self.u_flag;

        if self.eat('u') {
            if (u_flag && self.eat_regexp_unicode_surrogate_pair_escape())
                || self.eat_fixed_hex_digits(4)
                || (u_flag && self.eat_regexp_unicode_codepoint_escape())
            {
                return Ok(true);
            }
            if self.strict || u_flag {
                return Err("Invalid unicode escape");
            }
            self.rewind(start);
        }

        return Ok(false);
    }

    /// Eat the next characters as the following alternatives if possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    ///      LeadSurrogate `\u` TrailSurrogate
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_regexp_unicode_surrogate_pair_escape(&mut self) -> bool {
        let start = self.index();

        if self.eat_fixed_hex_digits(4) {
            let lead = self.last_int_value;
            if is_lead_surrogate(lead)
                && self.eat('\\')
                && self.eat('u')
                && self.eat_fixed_hex_digits(4)
            {
                let trail = self.last_int_value;
                if is_trail_surrogate(trail) {
                    self.last_int_value = combine_surrogate_pair(lead, trail);
                    return true;
                }
            }

            self.rewind(start);
        }

        return false;
    }

    /// Eat the next characters as the following alternatives if possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    ///      `{` CodePoint `}`
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_regexp_unicode_codepoint_escape(&mut self) -> bool {
        let start = self.index();

        if self.eat('{')
            && self.eat_hex_digits()
            && self.eat('}')
            && is_valid_unicode(self.last_int_value)
        {
            return true;
        }

        self.rewind(start);
        return false;
    }

    /// Eat the next characters as a RegExp `IdentityEscape` production if
    /// possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    /// IdentityEscape[U, N]::
    ///      [+U] SyntaxCharacter
    ///      [+U] `/`
    ///      [strict][~U] SourceCharacter but not UnicodeIDContinue
    ///      [annexB][~U] SourceCharacterIdentityEscape[?N]
    /// SourceCharacterIdentityEscape[N]::
    ///      [~N] SourceCharacter but not c
    ///      [+N] SourceCharacter but not one of c k
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_identity_escape(&mut self) -> bool {
        if let Some(cp) = self.code_point_with_offset(0) {
            if self.is_valid_identity_escape(cp) {
                // TODO: convert char to unicode code point
                self.last_int_value = cp as usize;
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn is_valid_identity_escape(&self, cp: char) -> bool {
        if self.u_flag {
            return is_syntax_character(cp) || cp == '/';
        } else if self.strict {
            //return !is_id_continue(cp);
            return false;
        } else if self.n_flag {
            return !(cp == 'c' || cp == 'k');
        }
        return cp != 'c';
    }

    /// Eat the next characters as a RegExp `DecimalEscape` production if
    /// possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    /// DecimalEscape::
    ///      NonZeroDigit DecimalDigits(opt) [lookahead ∉ DecimalDigit]
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_decimal_escape(&mut self) -> bool {
        self.last_int_value = 0;
        if let Some(cp) = self.code_point_with_offset(0) {
            if cp.is_digit(10) {
                self.last_int_value = 10 * self.last_int_value + cp.to_digit(10).unwrap() as usize;
                self.advance();
                /*do {
                    self.last_int_value = 10 * self.last_int_value + cp.to_digit(10);
                    self.advance();
                } while (
                    (cp = this.currentCodePoint) >= DigitZero &&
                    cp <= DigitNine
                )*/
                return true;
            }
        }
        return false;
    }

    /// Eat the next characters as a RegExp `UnicodePropertyValueExpression`
    /// production if possible.
    /// Set `this._lastKeyValue` and `this._lastValValue` if it ate the next
    /// characters successfully.
    /// ```grammar
    /// UnicodePropertyValueExpression::
    ///      UnicodePropertyName `=` UnicodePropertyValue
    ///      LoneUnicodePropertyNameOrValue
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_unicode_property_value_expression(&mut self) -> Result<bool, &str> {
        let start = self.index();

        // UnicodePropertyName `=` UnicodePropertyValue
        if self.eat_unicode_property_name() && self.eat('=') {
            self.last_key_value = self.last_str_value.clone();
            if self.eat_unicode_property_value() {
                self.last_val_value = self.last_str_value.clone();
                if is_valid_unicode_property(
                    self.ecma_version,
                    &self.last_key_value,
                    &self.last_val_value,
                ) {
                    return Ok(true);
                }
                return Err("Invalid property name");
            }
        }
        self.rewind(start);

        // LoneUnicodePropertyNameOrValue
        if self.eat_lone_unicode_property_name_or_value() {
            let name_or_value = self.last_str_value.clone();
            if is_valid_unicode_property(self.ecma_version, "General_Category", &name_or_value) {
                self.last_key_value = "General_Category".to_string();
                self.last_val_value = name_or_value;
                return Ok(true);
            }
            if is_valid_lone_unicode_property(self.ecma_version, &name_or_value) {
                self.last_key_value = name_or_value;
                self.last_val_value = "".to_string();
                return Ok(true);
            }
            return Err("Invalid property name");
        }
        Ok(false)
    }

    /// Eat the next characters as a RegExp `UnicodePropertyName` production if
    /// possible.
    /// Set `this._lastStrValue` if it ate the next characters successfully.
    /// ```grammar
    /// UnicodePropertyName::
    ///      UnicodePropertyNameCharacters
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_unicode_property_name(&mut self) -> bool {
        self.last_str_value = "".to_string();
        while let Some(cp) = self.code_point_with_offset(0) {
            if !is_unicode_property_name_character(cp) {
                break;
            }
            self.last_str_value.push(cp);
            self.advance();
        }
        self.last_str_value != ""
    }

    /// Eat the next characters as a RegExp `UnicodePropertyValue` production if
    /// possible.
    /// Set `this._lastStrValue` if it ate the next characters successfully.
    /// ```grammar
    /// UnicodePropertyValue::
    ///      UnicodePropertyValueCharacters
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_unicode_property_value(&mut self) -> bool {
        self.last_str_value = "".to_string();
        while let Some(cp) = self.code_point_with_offset(0) {
            if !is_unicode_property_value_character(cp) {
                break;
            }
            self.last_str_value.push(cp);
            self.advance();
        }
        self.last_str_value != ""
    }

    /// Eat the next characters as a RegExp `UnicodePropertyValue` production if
    /// possible.
    /// Set `this._lastStrValue` if it ate the next characters successfully.
    /// ```grammar
    /// LoneUnicodePropertyNameOrValue::
    ///      UnicodePropertyValueCharacters
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_lone_unicode_property_name_or_value(&mut self) -> bool {
        self.eat_unicode_property_value()
    }

    /// Eat the next characters as a `HexEscapeSequence` production if possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    /// HexEscapeSequence::
    ///      `x` HexDigit HexDigit
    /// HexDigit:: one of
    ///      0 1 2 3 4 5 6 7 8 9 a b c d e f A B C D E F
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_hex_escape_sequence(&mut self) -> Result<bool, &str> {
        let start = self.index();
        if self.eat('x') {
            if self.eat_fixed_hex_digits(2) {
                return Ok(true);
            }
            if self.u_flag || self.strict {
                return Err("Invalid escape");
            }
            self.rewind(start);
        }
        Ok(false)
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
        while let Some(cp) = self.code_point_with_offset(0) {
            if !cp.is_digit(10) {
                break;
            }
            self.last_int_value = 10 * self.last_int_value
                + self
                    .code_point_with_offset(0)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize;
            self.advance();
        }

        return self.index() != start;
    }

    /// Eat the next characters as a `HexDigits` production if possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    /// HexDigits::
    ///      HexDigit
    ///      HexDigits HexDigit
    /// HexDigit:: one of
    ///      0 1 2 3 4 5 6 7 8 9 a b c d e f A B C D E F
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_hex_digits(&mut self) -> bool {
        let start = self.index();
        self.last_int_value = 0;
        while let Some(cp) = self.code_point_with_offset(0) {
            if !cp.is_digit(16) {
                break;
            }
            self.last_int_value = 16 * self.last_int_value + cp.to_digit(16).unwrap() as usize;
            self.advance();
        }
        return self.index() != start;
    }

    /// Eat the next characters as a `HexDigits` production if possible.
    /// Set `self.last_int_value` if it ate the next characters successfully.
    /// ```grammar
    /// LegacyOctalEscapeSequence::
    ///      OctalDigit [lookahead ∉ OctalDigit]
    ///      ZeroToThree OctalDigit [lookahead ∉ OctalDigit]
    ///      FourToSeven OctalDigit
    ///      ZeroToThree OctalDigit OctalDigit
    /// OctalDigit:: one of
    ///      0 1 2 3 4 5 6 7
    /// ZeroToThree:: one of
    ///      0 1 2 3
    /// FourToSeven:: one of
    ///      4 5 6 7
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_legacy_octal_escape_sequence(&mut self) -> bool {
        if self.eat_octal_digit() {
            let n1 = self.last_int_value;
            if self.eat_octal_digit() {
                let n2 = self.last_int_value;
                if n1 <= 3 && self.eat_octal_digit() {
                    self.last_int_value = n1 * 64 + n2 * 8 + self.last_int_value
                } else {
                    self.last_int_value = n1 * 8 + n2;
                }
            } else {
                self.last_int_value = n1;
            }
            return true;
        }
        return false;
    }

    /// Eat the next characters as a `OctalDigit` production if possible.
    /// Set `this._lastIntValue` if it ate the next characters successfully.
    /// ```grammar
    /// OctalDigit:: one of
    ///      0 1 2 3 4 5 6 7
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_octal_digit(&mut self) -> bool {
        if let Some(cp) = self.code_point_with_offset(0) {
            if cp.is_digit(8) {
                self.advance();
                self.last_int_value = cp.to_digit(8).unwrap() as usize;
                return true;
            }
        }
        self.last_int_value = 0;
        return false;
    }

    /// Eat the next characters as the given number of `HexDigit` productions if
    /// possible.
    /// Set `self.last_int_value` if it ate the next characters successfully.
    /// ```grammar
    /// HexDigit:: one of
    ///      0 1 2 3 4 5 6 7 8 9 a b c d e f A B C D E F
    /// ```
    /// Returns `true` if it ate the next characters successfully.
    fn eat_fixed_hex_digits(&mut self, length: usize) -> bool {
        let start = self.index();
        self.last_int_value = 0;
        for _ in 0..length {
            let cp = self.code_point_with_offset(0);
            if cp.is_none() || !cp.unwrap().is_digit(16) {
                self.rewind(start);
                return false;
            }
            self.last_int_value =
                16 * self.last_int_value + cp.unwrap().to_digit(16).unwrap() as usize;
            self.advance();
        }
        return true;
    }

    fn count_capturing_parens(&mut self) -> u32 {
        let start = self.index();
        let mut in_class = false;
        let mut escaped = false;
        let mut count = 0;

        while let Some(cp) = self.code_point_with_offset(0) {
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
                && (self.code_point_with_offset(1) != Some('?')
                    || (self.code_point_with_offset(2) == Some('<')
                        && self.code_point_with_offset(3) != Some('=')
                        && self.code_point_with_offset(3) != Some('!')))
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
    fn count_capturing_parens_test() {
        let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
        let source = "foo|(abc)de";
        validator.reset(source, 0, source.len(), false);
        assert_eq!(validator.count_capturing_parens(), 1);
        let source = "foo|(?:abc)de";
        validator.reset(source, 0, source.len(), false);
        assert_eq!(validator.count_capturing_parens(), 0);
        let source = "((foo)|(abc)de)";
        validator.reset(source, 0, source.len(), false);
        assert_eq!(validator.count_capturing_parens(), 3);
    }
}
