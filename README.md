# `js-regexp`

[![Build Status](https://img.shields.io/travis/qkniep/regexpp-rs?logo=travis)](https://travis-ci.org/qkniep/regexpp-rs)
[![codecov](https://codecov.io/gh/qkniep/regexpp-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/qkniep/regexpp-rs)
![LoC](https://tokei.rs/b1/github/qkniep/regexpp-rs?category=code)
[![License](https://img.shields.io/github/license/qkniep/regexpp-rs)](LICENSE)

Rust port of [regexpp](https://github.com/mysticatea/regexpp) (ECMAScript regular expression parser).
This crate is being developed mainly as an efficient validator of ECMAScript regular expressions for
the [deno_lint](https://github.com/denoland/deno_lint) project.

## Using the Library

```rust
let mut validator = EcmaRegexValidator::new(EcmaVersion::ES2018);
assert_eq!(validator.validate_pattern("foo|abc(d)?", false), Ok(()));
assert_eq!(validator.validate_flags("gim", false), Ok(()));
```

## Performance

TBA

## License

Released under the [MIT License](LICENSE).
