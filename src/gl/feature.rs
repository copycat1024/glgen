use crate::xml::Node;
use std::collections::HashSet;

pub struct Feature {
    pub enums: HashSet<String>,
    pub commands: HashSet<String>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            enums: HashSet::new(),
            commands: HashSet::new(),
        }
    }

    pub fn process(&mut self, node: Node) {
        match node.name.as_str() {
            "require" => self.add(node),
            "remove" => self.remove(node),
            _ => {}
        }
    }

    fn add(&mut self, node: Node) {
        for mut node in node.children.into_iter() {
            let name = node.attr.remove("name").unwrap();
            match node.name.as_str() {
                "enum" => {
                    self.enums.insert(name);
                }
                "command" => {
                    self.commands.insert(name);
                }
                _ => {}
            }
        }
    }

    fn remove(&mut self, node: Node) {
        for node in node.children.into_iter() {
            let name = &node.attr["name"];
            match node.name.as_str() {
                "enum" => {
                    self.enums.remove(name);
                }
                "command" => {
                    self.commands.remove(name);
                }
                _ => {}
            }
        }
    }
}
