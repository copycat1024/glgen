use super::write_args;
use crate::ffi::Command;
use std::io::{Result, Write};

pub fn write_type(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    write!(out, "type {}T = extern \"system\" fn(", cmd.upper_name)?;
    write_args(out, cmd, |arg| arg.type_arg())?;
    writeln!(out, "){};", cmd.ret_type.as_ret())
}

pub fn write_field(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    writeln!(out, "{}_p: {}T,", cmd.lower_name, cmd.upper_name)
}

pub fn write_load(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    writeln!(
        out,
        "{}_p: load(&loader, \"{}\"),",
        cmd.lower_name, cmd.name
    )
}

pub fn write_wrap(out: &mut dyn Write, cmd: &Command) -> Result<()> {
    write!(out, "pub fn {}(&self, ", cmd.lower_name)?;
    write_args(out, cmd, |arg| {
        format!("{}: {}", arg.name(), arg.type_wrap())
    })?;
    writeln!(out, "){} {{", cmd.ret_type.as_ret())?;

    write!(out, "(self.{}_p)(", cmd.lower_name)?;
    write_args(out, cmd, |arg| arg.name())?;
    writeln!(out, ")")?;

    writeln!(out, "}}")?;
    writeln!(out)
}
