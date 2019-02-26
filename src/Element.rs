use crate::Node::*;


pub struct Element {
    pub nodeName: DOMString,
    // pub parentNode: Option<&'a Box<dyn Parent>>, // Box<dyn NodeInterface>
    pub childNodes: Vec<Box<dyn Children>>
}

impl Node for Element {
    fn nodeType(&self) -> NodeType {
        NodeType::ELEMENT_NODE
    }

    fn nodeName(&self) -> DOMString {
        self.nodeName.clone()
    }
}

impl Children for Element {
    // fn parentNode(&self) -> Option<&'a Box<dyn Parent>> {
    //     self.parentNode
    // }
}

impl Parent for Element {
    // fn childNodes(&self) -> Vec<Box<dyn Children>> {
    //     self.childNodes
    // }

    fn appendChild(&mut self, node: Box<dyn Children>) -> Option<&Box<dyn Children>> {
        self.childNodes.push(node);
        self.childNodes.last()
    }
}

impl Element {
    pub fn new(tagName: DOMString) -> Box<Element> {
        Box::new(
            Element {
                nodeName: tagName,
                // parentNode: parent_node,
                childNodes: vec![]
            }
        )
    }
}