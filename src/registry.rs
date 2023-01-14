use crate::{command::Command, document::Document, enum_::Enum, feature::Feature, node::Node};

pub struct Registry {
    enums: Vec<Enum>,
    pub commands: Vec<Command>,
    pub feature: Feature,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            enums: Vec::new(),
            commands: Vec::new(),
            feature: Feature::new(),
        }
    }

    pub fn build(mut self, mut doc: Document) -> Self {
        let node = doc.nodes.swap_remove(0);

        for node in node.children.into_iter() {
            match node.name.as_str() {
                "enums" => self.collect_enum_list(node),
                "commands" => self.collect_command_list(node),
                "feature" => self.collect_feature(node, "3.3"),
                _ => (),
            }
        }

        self
    }

    fn collect_enum_list(&mut self, node: Node) {
        for node in node.children.into_iter() {
            if node.name == "enum" {
                let enum_ = Enum::new(node);
                self.enums.push(enum_);
            }
        }
    }

    fn collect_command_list(&mut self, node: Node) {
        for node in node.children.into_iter() {
            if node.name == "command" {
                let cmd = Command::new(node);
                self.commands.push(cmd);
            }
        }
    }

    fn collect_feature(&mut self, node: Node, target: &'static str) {
        let number = &node.attr["number"];
        for node in node.children.into_iter() {
            if target >= number {
                self.feature.process(node);
            }
        }
    }
}
