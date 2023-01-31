use super::Generator;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Result, Write},
};

pub fn gen_enum(gen: &Generator, output: &mut dyn Write) -> Result<()> {
    let mut consts = BTreeSet::new();

    for (_, group) in gen.groups.values() {
        for const_ in group.iter() {
            consts.insert(const_);
        }
    }

    writeln!(output, "use super::consts;")?;
    writeln!(output)?;

    for (name, (c_type, values)) in gen.groups.iter() {
        if !values.is_empty() {
            let mut alias = BTreeMap::new();
            let mut concrete = BTreeSet::new();

            for item in values {
                if gen.consts[item].alias.is_none() {
                    concrete.insert(item);
                }
            }

            for item in values {
                if let Some(ref a) = gen.consts[item].alias {
                    if concrete.contains(a) {
                        alias.insert(item, a);
                    } else {
                        concrete.insert(item);
                    }
                }
            }

            writeln!(output, "#[repr({c_type})]")?;
            writeln!(output, "pub enum {name}{{")?;

            for item in concrete.iter() {
                let cast = if c_type != "u32" {
                    format!(" as {c_type}")
                } else {
                    String::new()
                };
                writeln!(output, "{} = consts::{item}{},", to_pascal_case(item), cast)?;
            }

            writeln!(output, "}}")?;
            writeln!(output)?;

            if !alias.is_empty() {
                writeln!(output, "impl {name}{{")?;

                for (name, value) in alias {
                    writeln!(
                        output,
                        "pub const {}: Self = Self::{};",
                        to_pascal_case(name),
                        to_pascal_case(value)
                    )?;
                }

                writeln!(output, "}}")?;
                writeln!(output)?;
            }

            let cast = if c_type != "u32" { " as u32" } else { "" };
            writeln!(output, "impl From<{c_type}> for {name} {{")?;
            writeln!(output, "fn from(value: {c_type}) -> Self {{")?;
            writeln!(output, "match value{cast} {{")?;

            for item in concrete.iter() {
                writeln!(output, "consts::{item} => Self::{},", to_pascal_case(item))?;
            }

            writeln!(output, "v => panic!(\"Unknow value {{v}} for {name}\")")?;
            writeln!(output, "}}")?;
            writeln!(output, "}}")?;
            writeln!(output, "}}")?;
            writeln!(output)?;
        }
    }

    Ok(())
}

fn to_pascal_case(name: &str) -> String {
    let (name, _) = name.chars().fold((String::new(), false), |(mut s, f), c| {
        if c == '_' {
            (s, true)
        } else if f {
            s.push(c);
            (s, false)
        } else {
            s.extend(c.to_lowercase());
            (s, false)
        }
    });

    name[2..].to_owned()
}
