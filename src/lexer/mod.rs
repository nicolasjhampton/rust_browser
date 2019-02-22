use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TOKEN {
    TAG_START(String),
    END_TAG_START(String),
    TAG_END,
    SINGLE_TAG_END,
    ATTR((String, String)),
    BOOL_ATTR(String),
    TEXT(String)
}

pub fn format_attr(attribute: String) -> (String, String) {
    let pair: Vec<&str> = attribute.splitn(2, "=").collect();
    let key = String::from(pair[0]);
    let value = pair[1]
        .trim_start_matches("\"") // escape quotes
        .trim_end_matches("\"") // escape quotes
        .to_string();
    (key, value)
}

pub struct Lexer<'a> {
    pub source: Peekable<Chars<'a>>,
    pub inner_tag: bool
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Lexer {
        let mut chars = source.chars().peekable();
        Lexer {
            source: chars,
            inner_tag: false
        } 
    }

    pub fn next_token(&mut self) -> Option<TOKEN> {
        match self.eat_whitespace() {
            Some('<') => {
                self.inner_tag = true;
                // TAG_START or END_TAG_START
                if let Some(true) = self.is_next(&vec!['/']) {
                    self.source.next();
                    let string = self.collect_until(' ', vec![' ', '>']);
                    return Some(TOKEN::END_TAG_START(string))
                }
                let string = self.collect_until(' ', vec![' ', '>']);
                Some(TOKEN::TAG_START(string))
            },
            Some('>') => {
                self.inner_tag = false;
                // TAG_END
                Some(TOKEN::TAG_END)
            },
            Some('/') => {
                // SINGLE_TAG_END
                if let Some(true) = self.is_next(&vec!['>']) {
                    self.source.next();
                    self.inner_tag = false;
                    return Some(TOKEN::SINGLE_TAG_END)
                }
                None
            },
            Some(character) => {
                // TAG_NAME, ATTR, or TEXT
                match self.inner_tag {
                    // TEXT
                    false => {
                        let string = self.collect_until(character, vec!['<']);
                        Some(TOKEN::TEXT(string))
                    },
                    true => {
                        // ATTR or BOOL_ATTR
                        let string = self.collect_until(character, vec![' ', '>']);
                        if string.contains("=") {
                            Some(TOKEN::ATTR(format_attr(string)))
                        } else {
                            Some(TOKEN::BOOL_ATTR(string))
                        }
                    }
                }
            },
            None => None
        }
    }

    pub fn eat_whitespace(&mut self) -> Option<char> {
        match self.source.next() {
            Some(' ') => self.eat_whitespace(),
            Some('\n') => self.eat_whitespace(),
            Some(character) => Some(character),
            None => None
        }
    }

    pub fn pop_char(&mut self) -> Option<char> {
        match self.source.next() {
            Some(value) => Some(value),
            None => None
        }
    }

    pub fn is_next(&mut self, comp: &Vec<char>) -> Option<bool> {
        match self.source.peek() {
            Some(value) => Some(comp.contains(value)),
            None => None
        }
    }

    pub fn collect_until(&mut self, character: char, end_chars: Vec<char>) -> String {
        let mut string: String = if character == ' ' {
            String::new()
        } else {
            character.to_string()
        };
        while let Some(false) = self.is_next(&end_chars) {
            string.push(self.pop_char().unwrap_or_default())
        }
        string
    }
}