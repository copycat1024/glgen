use crate::ffi::{Argument, Command};
use std::io::{Result, Write};

pub fn write_args<F>(out: &mut dyn Write, cmd: &Command, cb: F) -> Result<()>
where
    F: Fn(&Argument) -> String,
{
    let mut iter = cmd.args.iter();
    if let Some(arg) = iter.next() {
        write!(out, "{}", cb(arg))?;
    }
    for arg in iter {
        write!(out, ", {}", cb(arg))?;
    }
    Ok(())
}
