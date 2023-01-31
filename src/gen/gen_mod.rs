use super::Generator;
use std::io::{Result, Write};

const MOD_RS: &str = "
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
mod cmd;
mod load;
pub mod consts;
pub mod enums;

use load::load;

pub use cmd::GlCmd;
";

pub fn gen_mod(_gen: &Generator, output: &mut dyn Write) -> Result<()> {
    write!(output, "{MOD_RS}")
}
