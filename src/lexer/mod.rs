use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TOKEN {
    TAG_START,
    TAG_NAME(String),
    END_TAG_START,
    TAG_END,
    SINGLE_TAG_END,
    ATTR(String, String),
    TEXT(String)
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
        match self.source.next() {
            Some('<') => {
                self.inner_tag = true;
                // TAG_START or END_TAG_START
                Some(TOKEN::TAG_START)
            },
            Some('>') => {
                self.inner_tag = false;
                // TAG_END or SINGLE_TAG_END
                Some(TOKEN::TAG_END)
            },
            Some(character) => {
                // TAG_NAME, ATTR, or TEXT
                match self.inner_tag {
                    // TEXT
                    false => self.collect_text(character),
                    true => {
                        // TAG_NAME or ATTR
                        self.collect_inner_tag(character)
                    }
                }
            },
            None => None
        }
    }

    pub fn pop_char_as_text(&mut self) -> Option<char> {
        match self.source.next() {
            Some(value) => Some(value),
            None => None
        }
    }

    pub fn is_next_tag_start(&mut self) -> Option<bool> {
        match self.source.peek() {
            Some(value) => Some(*value == '<'),
            None => None
        }
    }

    pub fn is_next_space(&mut self) -> Option<bool> {
        match self.source.peek() {
            Some(value) => Some(*value == ' ' || *value == '>'),
            None => None
        }
    }

    pub fn collect_text(&mut self, character: char) -> Option<TOKEN> {
        let mut string = character.to_string();
        while let Some(false) = self.is_next_tag_start() {
            string.push(self.pop_char_as_text().unwrap_or_default())
        }
        Some(TOKEN::TEXT(string))
    }

    pub fn collect_inner_tag(&mut self, character: char) -> Option<TOKEN> {
        let mut string = character.to_string();
        while let Some(false) = self.is_next_space() {
            string.push(self.pop_char_as_text().unwrap_or_default())
        }
        if string.contains("=") {
            let pair: Vec<&str> = string.splitn(2, "=").collect();
            let key = String::from(pair[0]);
            let value = String::from(pair[1]);
            Some(TOKEN::ATTR(key, value))
        } else {
            Some(TOKEN::TAG_NAME(string))
        }
    }
}