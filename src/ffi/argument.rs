use super::util;
use crate::{ffi::Type, gl::Binding};

pub struct Argument {
    type_: Type,
    name: String,
}

impl Argument {
    pub fn new(binding: Binding) -> Self {
        let Binding {
            type_,
            name,
            group,
            class,
            len,
        } = binding;
        println!("{group:?} {len:?} {class:?}");

        Self {
            type_: Type::new(&type_),
            name,
        }
    }

    pub fn type_c(&self) -> String {
        self.type_.c()
    }

    pub fn type_rust(&self) -> String {
        self.type_.rust()
    }

    pub fn ret_c(&self) -> String {
        if self.type_.not_void() {
            format!(" -> {}", self.type_.c())
        } else {
            String::new()
        }
    }

    pub fn ret_rust(&self) -> String {
        if self.type_.not_void() {
            format!(" -> {}", self.type_.rust())
        } else {
            String::new()
        }
    }

    pub fn name(&self) -> String {
        match self.name.as_ref() {
            "ref" => "ref_".to_owned(),
            "type" => "type_".to_owned(),
            name => name.chars().fold(String::new(), util::snake_case_reducer),
        }
    }
}
