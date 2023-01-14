use crate::xml::Node;
use xml::reader::{Error, XmlEvent};

pub struct Document {
    pub nodes: Vec<Node>,
}

impl Document {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn take<I>(mut self, mut iter: I) -> Self
    where
        I: Iterator<Item = Result<XmlEvent, Error>>,
    {
        loop {
            let item = iter.next();
            match item {
                Some(Ok(token)) => self.take_token(&mut iter, token),
                Some(Err(error)) => panic!("{}", error),
                None => break self,
            }
        }
    }

    fn take_token<I>(&mut self, iter: &mut I, token: XmlEvent)
    where
        I: Iterator<Item = Result<XmlEvent, Error>>,
    {
        if let XmlEvent::StartElement {
            name, attributes, ..
        } = token
        {
            let mut node = Node::new(name, attributes);
            node.take(iter);
            self.nodes.push(node)
        }
    }
}
