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

    pub fn type_arg(&self) -> String {
        self.type_.as_arg()
    }

    pub fn type_wrap(&self) -> String {
        self.type_.as_wrap()
    }

    pub fn name(&self) -> String {
        match self.name.as_ref() {
            "ref" => "ref_".to_owned(),
            "type" => "type_".to_owned(),
            name => name.chars().fold(String::new(), util::snake_case_reducer),
        }
    }
}
