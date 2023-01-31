use crate::gl::Enum;

pub struct Const {
    pub name: String,
    pub value: String,
    pub alias: Option<String>,
    pub type_: Option<String>,
}

impl Const {
    pub fn new(enum_: Enum) -> Self {
        let Enum {
            value,
            name,
            alias,
            type_,
            ..
        } = enum_;

        Self {
            name,
            value,
            alias,
            type_,
        }
    }
}
