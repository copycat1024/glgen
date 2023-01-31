use super::Generator;
use std::io::{Result, Write};

pub fn gen_const(gen: &Generator, output: &mut dyn Write) -> Result<()> {
    for c in gen.consts.values() {
        let type_ = if let Some("ull") = c.type_.as_deref() {
            "u64"
        } else {
            "u32"
        };
        writeln!(output, "pub const {}: {type_} = {};", c.name, c.value)?;
    }

    Ok(())
}
