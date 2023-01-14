mod gl;
mod xml;

use crate::{xml::Document, gl::Registry};
use ::xml::reader::{EventReader, ParserConfig};
use std::{fs::File, io::BufReader};

fn main() {
    use std::io::Write;

    let file = File::open("registry/xml/gl.xml").unwrap();
    let file = BufReader::new(file);
    let config = ParserConfig::new().trim_whitespace(false);

    let stream = EventReader::new_with_config(file, config).into_iter();
    let document = Document::new().take(stream);
    let registry = Registry::new().build(document);

    let mut src = File::create("source.txt").unwrap();
    for cmd in registry.feature.commands.iter() {
        writeln!(src, "{}", cmd).unwrap();
    }
}
