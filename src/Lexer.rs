#![allow(dead_code)]

use std::str::Chars;
use std::iter::Peekable;

use std::fs::File;
use std::io::prelude::*;

use crate::Token::TOKEN;


pub fn format_attr(attribute: String) -> (String, String) {
    let pair: Vec<&str> = attribute.splitn(2, "=").collect();
    let key = String::from(pair[0]);
    let x: &[_] = &['\\', '\"']; // escape quotes and escapes
    let value = pair[1]
        .trim_start_matches(x)
        .trim_end_matches(x)
        .to_string();
    (key, value)
}

pub struct Lexer<'a> {
    pub source: Box<Peekable<Chars<'a>>>,
    pub inner_tag: bool
}

impl<'a> Iterator for Lexer<'a> {

    type Item = TOKEN;

    fn next(&mut self) -> Option<TOKEN> {
        match self.eat_whitespace() {
            Some('<') => self.create_tag_name(),
            Some('>') => self.create_tag_end(), 
            Some('/') => self.create_single_tag_end(),
            Some(character) => {
                match self.inner_tag {
                    false => self.create_text(character),
                    true => self.create_attribute(character),
                }
            },
            None => None
        }
    }
}
//  &'a String
impl<'a> Lexer<'a> {
    pub fn new<'b>(source: &'b String) -> Lexer<'b> {
        let chars = source.chars().peekable();
        Lexer {
            source: Box::new(chars),
            inner_tag: false
        } 
    }

    pub fn from<'b>(path: &str, string: &'b mut String) -> Result<Lexer<'b>, std::io::Error> {
        match File::open(path) { //"src/lexer/index.html"
            Ok(mut source) => {
                match source.read_to_string(string) {
                    Ok(_) => {
                        let chars = string.chars().peekable();
                        Ok(Lexer {
                            source: Box::new(chars),
                            inner_tag: false 
                        })
                    },
                    Err(message) => Err(message)
                }
            },
            Err(message) => Err(message)
        }
    }

    pub fn create_tag_name(&mut self) -> Option<TOKEN> {
        self.inner_tag = true;
        // TAG_START or END_TAG_START
        if let Some(true) = self.is_next(&vec!['/']) {
            self.source.next();
            let string = self.collect_until(' ', vec![' ', '>']);
            return Some(TOKEN::END_TAG_START(string))
        }
        let string = self.collect_until(' ', vec![' ', '>']);
        Some(TOKEN::TAG_START(string))
    }

    pub fn create_tag_end(&mut self) -> Option<TOKEN> {
        self.inner_tag = false;
        // TAG_END
        Some(TOKEN::TAG_END)
    }

    pub fn create_single_tag_end(&mut self) -> Option<TOKEN> {
        // SINGLE_TAG_END
        if let Some(true) = self.is_next(&vec!['>']) {
            self.source.next();
            self.inner_tag = false;
            return Some(TOKEN::SINGLE_TAG_END)
        }
        None
    }

    pub fn create_text(&mut self, character: char) -> Option<TOKEN> {
        let string = self.collect_until(character, vec!['<']);
        Some(TOKEN::TEXT(string))
    }

    pub fn create_attribute(&mut self, character: char) -> Option<TOKEN> {
        // ATTR or BOOL_ATTR
        let string = self.collect_until(character, vec![' ', '>']);
        if string.contains("=") {
            Some(TOKEN::ATTR(format_attr(string)))
        } else {
            Some(TOKEN::BOOL_ATTR(string))
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

#[cfg(test)]
mod tests {
    use super::TOKEN;
    use super::Lexer;

    #[test]
    fn it_initializes_new_lexer() {
        let source = String::from("<body><div><p>This is a paragraph</p></div></body>");
        let lexer = Lexer::new(&source);
        assert_eq!(lexer.source.collect::<String>(), source);
    }

    #[test]
    fn it_creates_the_correct_tokens() {
        let source = String::from("
        <!DOCTYPE html>
        <html>
            <head>
                <link href=\"css/styles.css\" rel=\"stylesheet\" />
            </head>
            <body>
                <div>
                    <p hidden class=\"center\">This is a paragraph</p>
                </div>
            </body>
        </html>
        ");
        let lexer = Lexer::new(&source);
        let answers = vec![
            TOKEN::TAG_START("!DOCTYPE".to_string()),
            TOKEN::BOOL_ATTR("html".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("html".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("head".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("link".to_string()),
            TOKEN::ATTR(("href".to_string(), "css/styles.css".to_string())),
            TOKEN::ATTR(("rel".to_string(), "stylesheet".to_string())),
            TOKEN::SINGLE_TAG_END,
            TOKEN::END_TAG_START("head".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("body".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("p".to_string()),
            TOKEN::BOOL_ATTR("hidden".to_string()),
            TOKEN::ATTR(("class".to_string(), "center".to_string())),
            TOKEN::TAG_END,
            TOKEN::TEXT("This is a paragraph".to_string()),
            TOKEN::END_TAG_START("p".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("body".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("html".to_string()),
            TOKEN::TAG_END
        ];
        for (idx, token) in lexer.enumerate() {
            assert_eq!(token, answers[idx]);
        }
    }

    #[test]
    fn it_consumes_an_entire_string_into_tokens() {
        let answers = vec![
            TOKEN::TAG_START("!DOCTYPE".to_string()),
            TOKEN::BOOL_ATTR("html".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("html".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("head".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("link".to_string()),
            TOKEN::ATTR(("href".to_string(), "css/styles.css".to_string())),
            TOKEN::ATTR(("rel".to_string(), "stylesheet".to_string())),
            TOKEN::SINGLE_TAG_END,
            TOKEN::END_TAG_START("head".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("body".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::TAG_START("p".to_string()),
            TOKEN::BOOL_ATTR("hidden".to_string()),
            TOKEN::ATTR(("class".to_string(), "center".to_string())),
            TOKEN::TAG_END,
            TOKEN::TEXT("This is a paragraph".to_string()),
            TOKEN::END_TAG_START("p".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("div".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("body".to_string()),
            TOKEN::TAG_END,
            TOKEN::END_TAG_START("html".to_string()),
            TOKEN::TAG_END
        ];
        let mut source = String::new();
        let path = "src/index.html";
        if let Ok(lexer) = Lexer::from(path, &mut source) {
            assert_eq!(lexer.collect::<Vec<TOKEN>>(), answers);
        } else {
            assert!(false);
        }
    }
}