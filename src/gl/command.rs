use crate::{gl::Binding, xml::Node};

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
        self.proto = Binding::from_node(node);
    }

    fn add_param(&mut self, node: Node) {
        let param = Binding::from_node(node);
        self.param.push(param)
    }
}
