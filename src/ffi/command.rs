use super::util;
use crate::{
    ffi::{Argument, Type},
    gl::{self, Binding},
};
use std::io::{Result, Write};

pub struct Command {
    name: String,
    upper_name: String,
    lower_name: String,
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

    pub fn write_type(&self, file: &mut dyn Write) -> Result<()> {
        write!(file, "type {}T = extern \"system\" fn(", self.upper_name)?;
        self.write_args(file, |arg| arg.type_arg())?;
        writeln!(file, "){};", self.ret_type.as_ret())
    }

    pub fn write_field(&self, file: &mut dyn Write) -> Result<()> {
        writeln!(file, "{}_p: {}T,", self.lower_name, self.upper_name)
    }

    pub fn write_load(&self, file: &mut dyn Write) -> Result<()> {
        writeln!(
            file,
            "{}_p: load(loader, \"{}\"),",
            self.lower_name, self.name
        )
    }

    pub fn write_wrap(&self, file: &mut dyn Write) -> Result<()> {
        write!(file, "pub fn {}(&self, ", self.lower_name)?;
        self.write_args(file, |arg| format!("{}: {}", arg.name(), arg.type_arg()))?;
        writeln!(file, "){} {{", self.ret_type.as_ret())?;

        write!(file, "(self.{}_p)(", self.lower_name)?;
        self.write_args(file, |arg| arg.name())?;
        writeln!(file, ")")?;

        writeln!(file, "}}")?;
        writeln!(file)
    }

    fn write_args<F>(&self, output: &mut dyn Write, cb: F) -> Result<()>
    where
        F: Fn(&Argument) -> String,
    {
        let mut iter = self.args.iter();
        if let Some(arg) = iter.next() {
            write!(output, "{}", cb(arg))?;
        }
        for arg in iter {
            write!(output, ", {}", cb(arg))?;
        }
        Ok(())
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
