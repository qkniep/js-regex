// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

enum ESVersion {
    ES5,
    ES2015,
    ES2016,
    ES2017,
    ES2018,
    ES2019,
    ES2020,
    ES2021,
}

pub struct ESRegexValidator {
    es_version: ESVersion,
}

impl ESRegexValidator {
    fn new(es_version: ESVersion) -> Self {
        ESRegexValidator {
            es_version
        }
    }

    fn validate(&self, regex: &str) {
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
