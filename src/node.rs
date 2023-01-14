use std::collections::HashMap;
use xml::{
    attribute::OwnedAttribute,
    name::OwnedName,
    reader::{Error, XmlEvent},
};

pub enum Token {
    Node(usize),
    Text(String),
}

pub struct Node {
    pub name: String,
    pub attr: HashMap<String, String>,
    pub children: Vec<Node>,
    pub text: Vec<Token>,
}

impl Node {
    pub fn new(name: OwnedName, attr: Vec<OwnedAttribute>) -> Self {
        let name = name.local_name;
        let attr = attr.into_iter().fold(HashMap::new(), |mut dict, item| {
            let key = item.name.local_name;
            let value = item.value;
            dict.insert(key, value);
            dict
        });

        Self {
            name,
            attr,
            children: Vec::new(),
            text: Vec::new(),
        }
    }

    pub fn take<I>(&mut self, iter: &mut I)
    where
        I: Iterator<Item = Result<XmlEvent, Error>>,
    {
        loop {
            let item = iter.next();
            match item {
                Some(Ok(token)) => {
                    if self.take_token(iter, token) {
                        break;
                    }
                }
                Some(Err(error)) => {
                    println!("{}", error);
                    break;
                }
                None => {
                    unreachable!("Iterator terminated");
                }
            }
        }
    }

    fn take_token<I>(&mut self, iter: &mut I, token: XmlEvent) -> bool
    where
        I: Iterator<Item = Result<XmlEvent, Error>>,
    {
        match token {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let mut node = Node::new(name, attributes);
                let i = self.children.len();
                node.take(iter);
                self.children.push(node);
                self.text.push(Token::Node(i));
                false
            }
            XmlEvent::Characters(text) => {
                self.text.push(Token::Text(text));
                false
            }
            XmlEvent::EndElement { .. } => true,
            _ => false,
        }
    }
}
