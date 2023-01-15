use super::{
    expr::{write_field, write_load, write_type, write_wrap},
    Generator,
};
use std::io::{Result, Write};

const TYPES: &str = "
use super::load;
use std::ffi::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

type DebugProc = Option<
    extern \"system\" fn(
        source: c_uint,
        gltype: c_uint,
        id: c_uint,
        severity: c_uint,
        length: isize,
        message: *const c_char,
        userParam: *mut c_void,
    ),
>;

";

pub fn gen_cmd(gen: &Generator, output: &mut dyn Write) -> Result<()> {
    gen_cmd_types(gen, output)?;
    gen_cmd_struct(gen, output)?;
    gen_cmd_impl(gen, output)?;

    Ok(())
}

fn gen_cmd_types(gen: &Generator, output: &mut dyn Write) -> Result<()> {
    write!(output, "{TYPES}")?;

    for cmd in gen.commands.values() {
        write_type(output, cmd)?;
    }

    writeln!(output)
}

fn gen_cmd_struct(gen: &Generator, output: &mut dyn Write) -> Result<()> {
    writeln!(output, "pub struct GlCmd {{")?;

    for cmd in gen.commands.values() {
        write_field(output, cmd)?;
    }

    writeln!(output, "}}")?;
    writeln!(output)
}

fn gen_cmd_impl(gen: &Generator, output: &mut dyn Write) -> Result<()> {
    writeln!(output, "impl GlCmd {{")?;

    writeln!(output, "pub fn load<FnLoad>(loader: FnLoad) -> Self")?;
    writeln!(output, "where FnLoad: Fn(*const c_char) -> *mut c_void {{")?;
    writeln!(output, "Self {{")?;

    for cmd in gen.commands.values() {
        write_load(output, cmd)?;
    }

    writeln!(output, "}}")?;
    writeln!(output, "}}")?;

    for cmd in gen.commands.values() {
        write_wrap(output, cmd)?;
    }

    writeln!(output, "}}")?;
    writeln!(output)
}
