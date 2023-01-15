mod ffi;
mod gen;
mod gl;
// mod test;
mod xml;

use crate::{gen::Generator, gl::Registry, xml::Document};
use ::xml::reader::{EventReader, ParserConfig};
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("registry\\xml\\gl.xml").unwrap();
    let file = BufReader::new(file);
    let config = ParserConfig::new();

    let stream = EventReader::new_with_config(file, config).into_iter();
    let document = Document::new().take(stream);
    let registry = Registry::new().build(document);
    let generator = Generator::collect(registry, Some("src\\test"));
    generator.write().unwrap();
}
