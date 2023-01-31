use crate::xml::Node;

pub struct Enum {
    pub name: String,
    pub value: String,
    pub group: Vec<String>,
    pub alias: Option<String>,
    pub type_: Option<String>,
}

impl Enum {
    pub fn new(mut node: Node) -> Self {
        Self {
            name: node.attr.remove("name").unwrap(),
            value: node.attr.remove("value").unwrap(),
            group: node
                .attr
                .get("group")
                .map(|group| group.split(',').map(|s| s.into()).collect())
                .unwrap_or_default(),
            alias: node.attr.remove("alias"),
            type_: node.attr.remove("type"),
        }
    }
}
