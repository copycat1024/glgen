use super::util;
use crate::{ffi::Argument, gen::Generator, gl};

pub struct Command {
    pub decl: Argument,
    pub args: Vec<Argument>,

    pub upper: String,
    pub lower: String,
}

impl Command {
    pub fn new(cmd: gl::Command) -> Self {
        let gl::Command { proto, param } = cmd;

        let upper = Self::get_upper_name(&proto.name);
        let lower = Self::get_lower_name(&proto.name);
        let decl = Argument::new(proto);

        Self {
            decl,
            args: param.into_iter().map(Argument::new).collect(),
            upper,
            lower,
        }
    }

    pub fn type_name(&self) -> String {
        format!("{}T", self.upper)
    }

    pub fn field_name(&self) -> String {
        format!("{}_p", self.lower)
    }

    pub fn rust_name(&self) -> String {
        self.lower.to_owned()
    }

    pub fn gl_name(&self) -> String {
        self.decl.gl_name()
    }

    pub fn ret_c(&self) -> String {
        self.decl.ret_c()
    }

    pub fn ret_rust(&self, gen: &Generator) -> String {
        self.decl.ret_rust(gen)
    }


    pub fn get_upper_name(name: &str) -> String {
        let name = name.chars().fold(String::new(), util::pascal_case_reducer);
        name[2..].to_owned()
    }

    pub fn get_lower_name(name: &str) -> String {
        let name = name.chars().fold(String::new(), util::snake_case_reducer);
        name[3..].to_owned()
    }
}
