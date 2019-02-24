#![allow(dead_code)]

use crate::Token::TOKEN;

pub type DOMString<'a> = &'a str;
pub type LiveDOMNode = Box<dyn NodeInterface>;
pub type NodeList = Vec<LiveDOMNode>;

pub enum NodeType {
    ELEMENT_NODE = 1,
    ATTRIBUTE_NODE,
    TEXT_NODE,
    CDATA_SECTION_NODE,
    ENTITY_REFERENCE_NODE,
    ENTITY_NODE,
    PROCESSING_INSTRUCTION_NODE,
    COMMENT_NODE,
    DOCUMENT_NODE,
    DOCUMENT_TYPE_NODE,
    DOCUMENT_FRAGMENT_NODE,
    NOTATION_NODE
}

pub trait Node<'a> {
    const nodeName: DOMString<'a>;
    const nodeValue: Option<DOMString<'a>>;
    const nodeType: NodeType;
    // const parentNode: Option<Box<Node>>;
    // const childNodes: NodeList;
}

pub trait NodeInterface {
    // fn removeChild(&mut self, node: Box<dyn NodeInterface>) -> Result<(), String>;
    fn appendChild(&mut self, node: LiveDOMNode) -> Option<&LiveDOMNode>;
}