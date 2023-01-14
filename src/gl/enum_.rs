use crate::xml::Node;

pub struct Enum {
    pub group: Vec<String>,
}

impl Enum {
    pub fn new(node: Node) -> Self {
        Self {
            group: node
                .attr
                .get("group")
                .map(|group| group.split(',').map(|s| s.into()).collect())
                .unwrap_or_default(),
        }
    }
}
