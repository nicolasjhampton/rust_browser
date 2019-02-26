use crate::Node::*;

pub struct Document {
    childNodes: Vec<Box<dyn Children>>
}

impl Node for Document {
    fn nodeType(&self) -> NodeType {
        NodeType::DOCUMENT_NODE
    }

    fn nodeName(&self) -> DOMString {
        String::from("#document")
    }
}

impl Parent for Document {
    // fn childNodes(&self) -> Vec<Box<dyn Children>> {
    //     self.childNodes
    // }

    fn appendChild(&mut self, node: Box<dyn Children>) -> Option<&Box<dyn Children>> {
        self.childNodes.push(node);
        self.childNodes.last()
    }
}

impl Document {
    pub fn new() -> Box<Document> {
        Box::new(
            Document {
                childNodes: vec![]
            }
        )
    }
}
