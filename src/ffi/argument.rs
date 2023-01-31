use super::util;
use crate::{ffi::Type, gen::Generator, gl::Binding};

pub struct Argument {
    type_: Type,
    name: String,
    pub group: Option<String>,
}

impl Argument {
    pub fn new(binding: Binding) -> Self {
        let Binding {
            type_, name, group, ..
        } = binding;

        Self {
            type_: Type::new(&type_),
            name,
            group,
        }
    }

    pub fn type_c(&self) -> String {
        self.type_.c()
    }

    pub fn type_rust_raw(&self) -> String {
        self.type_.match_rust().to_owned()
    }

    pub fn type_rust(&self, gen: &Generator) -> String {
        match self.group(gen) {
            Some(group) => format!("{}enums::{group}", self.type_.mod_()),
            None => self.type_.rust(),
        }
    }

    pub fn type_cast(&self, gen: &Generator) -> String {
        match self.group(gen) {
            Some(_) => format!(" as {}", self.type_.rust()),
            None => String::new(),
        }
    }

    pub fn ret_c(&self) -> String {
        if self.type_.not_void() {
            format!(" -> {}", self.type_c())
        } else {
            String::new()
        }
    }

    pub fn ret_rust(&self, gen: &Generator) -> String {
        if self.type_.not_void() {
            format!(" -> {}", self.type_rust(gen))
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

    pub fn gl_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn group(&self, gen: &Generator) -> Option<&str> {
        self.group
            .as_deref()
            .and_then(|name| gen.has_group(name).then_some(name))
    }

    pub fn is_grouped(&self, gen: &Generator) -> bool {
        self.group(gen).is_some()
    }
}
