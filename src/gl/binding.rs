use crate::xml::{Node, Token};

pub struct Binding {
    pub type_: String,
    pub name: String,
    pub group: Option<String>,
    pub len: Option<String>,
    pub class: Option<String>,
}

impl Binding {
    pub fn new() -> Self {
        Self {
            type_: String::new(),
            name: String::new(),
            group: None,
            len: None,
            class: None,
        }
    }

    pub fn from_node(node: Node) -> Self {
        let Node {
            children,
            text,
            mut attr,
            ..
        } = node;
        let mut type_ = String::new();
        let mut name = String::new();

        let group = attr.remove("group");
        let len = attr.remove("group");
        let class = attr.remove("class");

        for token in text {
            match token {
                Token::Node(i) => {
                    if let Some(Token::Text(s)) = children[i].text.get(0) {
                        let tag = &children[i].name;
                        if tag == "name" {
                            name = s.into();
                            break;
                        } else if tag == "ptype" {
                            type_ += s;
                        }
                    }
                }
                Token::Text(s) => {
                    type_ += &s;
                }
            }
        }

        Self {
            type_,
            name,
            group,
            len,
            class,
        }
    }
}
