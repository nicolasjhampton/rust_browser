#![allow(dead_code)]

use crate::Token::TOKEN;
use crate::Lexer::Lexer;
use crate::Document::Document;
use crate::Node::*;

pub struct Parser {
    pub tokens: Vec<TOKEN>
}

impl Parser {
    pub fn new(tokens: Vec<TOKEN>) -> Parser {
        Parser {
            tokens
        }
    }

    pub fn parse(&mut self) -> Document {
        let document = Document::new();
        if let Some(token) = self.tokens.pop() {
            let node = self.parse_token(token);
            document.appendChild(node);
        }
        document
    }

    pub fn parse_token(&mut self, token: TOKEN) -> LiveDOMNode {
        match token {
            TOKEN::TAG_START(tagName) => ,
            TOKEN::END_TAG_START(tagName) => ,
            TOKEN::TAG_END => ,
            TOKEN::SINGLE_TAG_END => ,
            TOKEN::ATTR((key, value)) => ,
            TOKEN::BOOL_ATTR(attribute) => ,
            TOKEN::TEXT(text) => ,
            None => 
        }
    }

    // pub next(&mut self) -> 
}

#[cfg(test)]
mod tests {
    use super::TOKEN;
    use super::Lexer;
    use super::Parser;

    #[test]
    fn it_initializes_new_parser() {
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
        let mut tokens: Vec<TOKEN>;
        let mut source = String::new();
        let path = "src/index.html";
        if let Ok(mut lexer) = Lexer::from(path, &mut source) {
            tokens = lexer.collect();
            let parser = Parser::new(tokens);
            assert_eq!(answers, parser.tokens);
        }
    }
}