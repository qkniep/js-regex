// Ported from: https://github.com/mysticatea/regexpp
//
// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

extern crate regexpp_rs;

use regexpp_rs::{EcmaRegexValidator, EcmaVersion};

#[test]
fn basic_invalid() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/basic-invalid.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES5);
    assert_ne!(validator.validate_pattern("(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(*)", false), Ok(()));
    assert_ne!(validator.validate_pattern("+", false), Ok(()));
    assert_ne!(validator.validate_pattern("?", false), Ok(()));
    assert_ne!(validator.validate_pattern(")", false), Ok(()));
    assert_ne!(validator.validate_pattern("[", false), Ok(()));
    assert_ne!(validator.validate_pattern("^*", false), Ok(()));
    assert_ne!(validator.validate_pattern("$*", false), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2}", false), Ok(()));
    assert_ne!(validator.validate_pattern("${2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("\\2(a)(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[a-b--+]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0001-\\u0000]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{1}-\\u{2}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{2}-\\u{1}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\z-\\a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[0-9--+]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c-a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[🌷-🌸]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[🌸-🌷]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\uD834\\uDF06-\\uD834\\uDF08a-z]", false), Ok(()));
}

#[test]
fn basic_invalid_2015() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/basic-invalid-2015.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2015);
    assert_ne!(validator.validate_pattern("(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?=foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?!foo", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(a{2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(*)", false), Ok(()));
    assert_ne!(validator.validate_pattern("+", false), Ok(()));
    assert_ne!(validator.validate_pattern("?", false), Ok(()));
    assert_ne!(validator.validate_pattern(")", false), Ok(()));
    assert_ne!(validator.validate_pattern("[", false), Ok(()));
    assert_ne!(validator.validate_pattern("^*", false), Ok(()));
    assert_ne!(validator.validate_pattern("$*", false), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2}", false), Ok(()));
    assert_ne!(validator.validate_pattern("${2,1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("\\2(a)(", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(:a", false), Ok(()));
    assert_ne!(validator.validate_pattern("[b-a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[a-b--+]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0001-\\u0000]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{1}-\\u{2}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{2}-\\u{1}]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\z-\\a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[0-9--+]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c-a]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[🌷-🌸]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-🌸-\\u0000]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\ud83c\\udf38-\\u0000]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[🌸-🌷]", false), Ok(()));
    assert_ne!(validator.validate_pattern("[\\uD834\\uDF06-\\uD834\\uDF08a-z]", false), Ok(()));
}

#[test]
fn basic_invalid_2015_unicode() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/basic-invalid-2015-u.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2015);
    assert_ne!(validator.validate_pattern("(", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=foo", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?!", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?!foo", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a)*", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a)+", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a)?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){a}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){1,}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?=a){1,2}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{a}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,2", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1", true), Ok(()));
    assert_ne!(validator.validate_pattern("(a{2,1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{a}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{1,2?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1}?", true), Ok(()));
    assert_ne!(validator.validate_pattern("a{2,1?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(*)", true), Ok(()));
    assert_ne!(validator.validate_pattern("+", true), Ok(()));
    assert_ne!(validator.validate_pattern("?", true), Ok(()));
    assert_ne!(validator.validate_pattern(")", true), Ok(()));
    assert_ne!(validator.validate_pattern("[", true), Ok(()));
    assert_ne!(validator.validate_pattern("]", true), Ok(()));
    assert_ne!(validator.validate_pattern("{", true), Ok(()));
    assert_ne!(validator.validate_pattern("}", true), Ok(()));
    assert_ne!(validator.validate_pattern("^*", true), Ok(()));
    assert_ne!(validator.validate_pattern("$*", true), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2", true), Ok(()));
    assert_ne!(validator.validate_pattern("${1,2}", true), Ok(()));
    assert_ne!(validator.validate_pattern("${2,1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\1", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\2(a)(", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a)\\1", true), Ok(()));
    assert_ne!(validator.validate_pattern("(a)\\2", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a)\\2", true), Ok(()));
    assert_ne!(validator.validate_pattern("(a)(a)(a)(a)(a)(a)(a)(a)(a)(a)\\11", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?a", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?:a", true), Ok(()));
    assert_ne!(validator.validate_pattern("(:a", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\c1", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\c", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u1", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u12", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u123", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{z", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{20", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\u{110000}", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\377", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\400", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\a", true), Ok(()));
    assert_ne!(validator.validate_pattern("[b-a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[a-b--+]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c1]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\x]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\xz]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\x1]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u1]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u12]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u123]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{z]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{20]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{110000}]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\77]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\377]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\400]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\d-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\D-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\s-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\S-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\w-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\W-\\uFFFF]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\d]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\D]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\s]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\S]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\w]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0000-\\W]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u0001-\\u0000]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{2}-\\u{1}]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\u{2-\\u{1}]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\a-\\z]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\z-\\a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[0-9--+]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c-a]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c0-]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\c_]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[🌸-🌷]", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\d][\\12-\\14]{1,}[^\\d]", true), Ok(()));
}

#[test]
fn lookbehind_assertion_invalid_2017() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/lookbehind-assertion-invalid-2017.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2017);
    assert_ne!(validator.validate_pattern("(?<a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)", true), Ok(()));
}

#[test]
fn lookbehind_assertion_invalid_2018() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/lookbehind-assertion-invalid-2018.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("(?<a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)+", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)+", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)*", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a)*", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a){1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<=a){1}", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)?", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)?", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)+", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)+", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)*", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a)*", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a){1}", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<!a){1}", true), Ok(()));
}

#[test]
fn named_capturing_group_invalid_2017() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/named-capturing-group-invalid-2017.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2017);
    assert_ne!(validator.validate_pattern("\\k", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\k<a>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a>", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a>", true), Ok(()));
}

#[test]
fn named_capturing_group_invalid_2018() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/named-capturing-group-invalid-2018.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("(?a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("\\k", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\k<a>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<a", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\2", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<b>", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)\\k<b>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)(?<a>a)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)(?<a>a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)(?<\\u{61}>a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<a>a)(?<\\u0061>a)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<☀>a)\\k<☀>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<\\u0020>a)\\k<\\u0020>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<\\u0061\\u0062\\u0063>a)\\k<abd>", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<11>a)\\k<11>", true), Ok(()));
}

#[test]
fn unicode_group_names_invalid_2020() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/unicode-group-names-invalid.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2020);
    assert_ne!(validator.validate_pattern("(?<\\ud83d\\ude80>.)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<\\ud83d\\ude80>.)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<\\u{1f680}>.)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<\\u{1f680}>.)", true), Ok(()));
    assert_ne!(validator.validate_pattern("(?<🚀>.)", false), Ok(()));
    assert_ne!(validator.validate_pattern("(?<🚀>.)", true), Ok(()));
}

#[test]
fn unicode_property_escape_invalid_2017() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/unicode-property-escape-invalid-2017.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2017);
    assert_ne!(validator.validate_pattern("\\p", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{ASCII", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{ASCII}", true), Ok(()));
}

#[test]
fn unicode_property_escape_invalid_2018() {
    // source: https://github.com/mysticatea/regexpp/blob/master/test/fixtures/parser/literal/unicode-property-escape-invalid-2018.json
    let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
    assert_ne!(validator.validate_pattern("\\p", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{ASCII", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{General_Category}", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{General_Category=}", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{General_Category", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{General_Category=", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{General_Category=Letter", true), Ok(()));
    assert_ne!(validator.validate_pattern("\\p{General_Category=Hiragana}", true), Ok(()));
    assert_ne!(validator.validate_pattern("[\\p{Script=Hiragana}-\\p{Script=Katakana}]", true), Ok(()));
}
