use super::util;
use crate::{
    ffi::{Argument, Type},
    gl::{self, Binding},
};

pub struct Command {
    pub name: String,
    pub upper_name: String,
    pub lower_name: String,
    pub ret_type: Type,
    pub args: Vec<Argument>,
}

impl Command {
    pub fn new(cmd: gl::Command) -> Self {
        let gl::Command {
            proto: Binding { type_, name, .. },
            param,
        } = cmd;

        Self {
            name: name.clone(),
            upper_name: Self::get_upper_name(&name),
            lower_name: Self::get_lower_name(&name),
            ret_type: Type::new(&type_),
            args: param.into_iter().map(Argument::new).collect(),
        }
    }

    pub fn get_upper_name(name: &str) -> String {
        let name = name.chars().fold(String::new(), util::pascal_case_reducer);
        name[2..].into()
    }

    pub fn get_lower_name(name: &str) -> String {
        let name = name.chars().fold(String::new(), util::snake_case_reducer);
        name[3..].into()
    }
}
