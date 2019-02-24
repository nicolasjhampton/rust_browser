use crate::Node::*;
use crate::Token::TOKEN;

pub struct Document {
    pub parentNode: Option<LiveDOMNode>, // Box<dyn NodeInterface>
    pub childNodes: NodeList
}

impl<'a> Node<'a> for Document {
    const nodeName: DOMString<'a> = "#document";
    const nodeValue: Option<DOMString<'a>> = None;
    const nodeType: NodeType = NodeType::DOCUMENT_NODE;
}

impl NodeInterface for Document {
    // fn removeChild(&mut self, node: Box<dyn NodeInterface>) -> Result<(), String> {
    //     Ok(())
    // }

    fn appendChild(&mut self, node: LiveDOMNode) -> Option<&LiveDOMNode> {
        self.childNodes.push(node);
        self.childNodes.last()
    }
}

impl Document {
    pub fn new() -> Document {
        Document {
            parentNode: None,
            childNodes: vec![]
        }
    }
}
