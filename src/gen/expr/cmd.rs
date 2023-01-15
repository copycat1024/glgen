use super::write_args;
use crate::ffi::Command;
use std::io::{Result, Write};

pub fn write_type(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    write!(out, "type {} = extern \"system\" fn(", cmd.type_name())?;
    write_args(out, cmd, |arg| arg.type_c())?;
    writeln!(out, "){};", cmd.ret_c())
}

pub fn write_field(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    writeln!(out, "{}: {},", cmd.field_name(), cmd.type_name())
}

pub fn write_load(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    let field_name = cmd.field_name();
    let gl_name = cmd.gl_name();
    writeln!(out, "{field_name}: load(&loader, \"{gl_name}\"),",)
}

pub fn write_wrap(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    write!(out, "pub fn {}(&self, ", cmd.rust_name())?;
    write_args(out, cmd, |arg| {
        format!("{}: {}", arg.name(), arg.type_rust())
    })?;
    writeln!(out, "){} {{", cmd.ret_rust())?;

    write!(out, "(self.{}_p)(", cmd.field_name())?;
    write_args(out, cmd, |arg| arg.name())?;
    writeln!(out, ")")?;

    writeln!(out, "}}")?;
    writeln!(out)
}
