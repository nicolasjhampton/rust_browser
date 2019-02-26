#![allow(dead_code)]

use crate::Token::TOKEN;
use crate::Lexer::Lexer;
use crate::Document::Document;
use crate::Element::Element;
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

    pub fn parse(&mut self) -> LiveDOMNode {
        let mut document = Document::new();
        // let mut doc_pointer = Box::new(document);
        if let Some(token) = self.tokens.pop() {
            let element = self.create_element(String::from("html"));
            document.appendChild(element);
            // if let Ok(node) = self.parse_token(token, doc_pointer) {
            //     println!("{:?}", ((*(*node))).nodeName);
            // }
        }
        document
    }

    // pub fn parse_token<'a>(&mut self, token: TOKEN, parent_node: LiveDOMNode) -> Result<&LiveDOMNode, &str> {
    //     match token {
    //         TOKEN::TAG_START(tagName) => {
    //             let element = self.create_element(tagName, parent_node);
    //             let live_node = parent_node.appendChild(Box::new(element));
    //             Ok(live_node.unwrap())
    //         },
    //         // TOKEN::TEXT(text) => ,
    //         _ => Err("parsing error")
    //     }
    // }

    // TOKEN::END_TAG_START(tagName) => ,
    // TOKEN::TAG_END => ,
    // TOKEN::SINGLE_TAG_END => ,
    // TOKEN::ATTR((key, value)) => ,
    // TOKEN::BOOL_ATTR(attribute) => ,



    pub fn create_element(&mut self, tagName: String) -> Box<Element> {
        Element::new(tagName)
    }

    // pub next(&mut self) -> 
}

#[cfg(test)]
mod tests {
    use super::TOKEN;
    use super::Lexer;
    use super::Parser;
    use super::Document;

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
            let mut parser = Parser::new(tokens);
            assert_eq!(answers, parser.tokens);
            let doc = parser.parse();
            assert_eq!(doc.nodeName(), String::from("#document"));
            // assert_eq!((doc as Document).childNodes.len(), 1);
        }
    }
}