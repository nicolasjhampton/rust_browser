#![allow(dead_code)]

pub type DOMString = String;

pub type LiveDOMNode = Box<dyn Node>;
// pub type NodeList= Vec<>;

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

pub trait Node {
    fn nodeType(&self) -> NodeType;
    fn nodeName(&self) -> DOMString;
    fn nodeValue(&self) -> Option<DOMString>;
    fn childNodes(&self) -> Option<Vec<LiveDOMNode>>;
    fn appendChild(&mut self, node: LiveDOMNode) -> Option<&LiveDOMNode>;
}

pub trait NodeValue: Node {
    
}

pub trait Children: Node {
    // fn parentNode(&self) -> Option<Box<dyn Parent>>;
}

pub trait Parent: Node {
    // fn childNodes(&self) -> Vec<Box<dyn Children>>;
    // fn removeChild(&mut self, node: Box<dyn NodeInterface>) -> Result<(), String>;
    
}

