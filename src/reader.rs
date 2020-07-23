// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

const legacyImpl = {
    at(s: string, end: number, i: number): number {
        return i < end ? s.charCodeAt(i) : -1
    },
    width(c: number): number {
        return 1
    },
}

const unicodeImpl = {
    at(s: string, end: number, i: number): number {
        return i < end ? s.codePointAt(i)! : -1
    },
    width(c: number): number {
        return c > 0xffff ? 2 : 1
    },
}

struct Reader {
    implem = legacyImpl;
    src: &str,
    index: usize,
    end: usize,
    cp1: Option<char>,
    w1: usize,
    cp2: Option<char>,
    w2: usize,
    cp3: Option<char>,
    w3: usize,
    cp4: Option<char>,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            implem: legacyImpl,
            src: "",
            index: 0,
            end: 0,
            cp1: None,
            w1: 1,
            cp2: None,
            w2: 1,
            cp3: None,
            w3: 1,
            cp4: None,
            w4: 1,
        }
    }

    pub fn source(&self) -> &str {
        self.src
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn currentCodePoint(&self) -> char {
        self.cp1
    }

    pub fn nextCodePoint() -> char {
        self.cp2
    }

    pub fn nextCodePoint2() -> char {
        self.cp3
    }

    pub fn nextCodePoint3() -> char {
        self.cp4
    }

    pub fn reset(
        &mut self,
        source: &str,
        start: usize,
        end: usize,
        uFlag: bool,
    ) {
        self.implem = uFlag ? unicodeImpl : legacyImpl;
        self.src = source;
        self.end = end;
        self.rewind(start);
    }

    pub fn rewind(&mut self, index: usize) {
        let implem = self.implem;
        self.index = index;
        self.cp1 = implem.at(self.src, self.end, index);
        self.w1 = implem.width(this.cp1);
        self.cp2 = implem.at(self.src, self.end, index + self.w1);
        self.w2 = implem.width(this.cp2);
        self.cp3 = implem.at(self.src, self.end, index + self.w1 + self.w2);
        self.w3 = implem.width(self.cp3);
        self.cp4 = implem.at(
            self.src,
            self.end,
            index + self.w1 + self.w2 + self.w3,
        );
    }

    pub fn advance(&mut self) {
        if self.cp1 != -1 {
            let implem = self.implem;
            self.index += self.w1;
            self.cp1 = self.cp2;
            self.w1 = self.w2;
            self.cp2 = self.cp3;
            self.w2 = implem.width(self.cp2);
            self.cp3 = self.cp4;
            self.w3 = implem.width(self.cp3);
            self.cp4 = implem.at(
                self.src,
                self.end,
                self.index + self.w1 + self.w2 + self.w3,
            );
        }
    }

    pub fn eat(&mut self, cp: char) -> bool {
        if self.cp1 == cp {
            self.advance();
            return true;
        }
        return false;
    }

    pub fn eat2(&mut self, cp1: char, cp2: char) -> bool {
        if self.cp1 == cp1 && self.cp2 == cp2 {
            self.advance();
            self.advance();
            return true;
        }
        return false;
    }

    pub fn eat3(&mut self, cp1: char, cp2: char, cp3: char) -> bool {
        if self.cp1 == cp1 && self.cp2 == cp2 && self.cp3 == cp3 {
            self.advance();
            self.advance();
            self.advance();
            return true;
        }
        return false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eat_test() {
        let reader = Reader::new();
        reader.reset("abcdefghijk", 0, 11, true);
        assert_eq!(reader.eat('a'), true);
        assert_eq!(reader.eat3('b', 'd', 'd'), false);
        assert_eq!(reader.eat3('b', 'c', 'd'), true);
        assert_eq!(reader.eat2('e', 'f'), true);
        assert_eq!(reader.eat('h'), false);
        assert_eq!(reader.eat('g'), true);
        assert_eq!(reader.eat2('h', 'i'), true);
        assert_eq!(reader.eat3('j', 'k', 'a'), false);
    }
}
