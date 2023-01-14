use crate::node::{Node, Token};

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
}

pub struct Command {
    pub proto: Binding,
    pub param: Vec<Binding>,
}

impl Command {
    pub fn new(node: Node) -> Self {
        let mut me = Self {
            proto: Binding::new(),
            param: Vec::new(),
        };

        for node in node.children.into_iter() {
            match node.name.as_str() {
                "proto" => me.set_proto(node),
                "param" => me.add_param(node),
                _ => {}
            }
        }

        me
    }

    fn set_proto(&mut self, node: Node) {
        self.proto = Self::get_binding(node);
    }

    fn add_param(&mut self, node: Node) {
        let param = Self::get_binding(node);
        self.param.push(param)
    }

    fn get_binding(node: Node) -> Binding {
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
                            type_ += "_";
                            type_ += s;
                        } else {
                            panic!("Wrong tag in binding: {}", tag)
                        }
                    }
                }
                Token::Text(s) => {
                    type_ += &s;
                }
            }
        }

        Binding {
            type_,
            name,
            group,
            len,
            class,
        }
    }
}
