use super::{gen_cmd, gen_const, gen_enum, gen_load, gen_mod};
use crate::{
    ffi::{Command, Const},
    gl::Registry,
};
use std::{
    collections::{btree_map, BTreeMap},
    fs::{create_dir_all, File},
    io::{Result, Write},
    ops::Deref,
    path::{Path, PathBuf},
};

pub struct Generator {
    pub commands: BTreeMap<String, Command>,
    pub consts: BTreeMap<String, Const>,
    pub groups: BTreeMap<String, (String, Vec<String>)>,
    path: PathBuf,
}

impl Generator {
    pub fn collect<P: AsRef<Path>>(registry: Registry, path: Option<P>) -> Self {
        let Registry {
            feature,
            commands: gl_cmd,
            enums,
        } = registry;
        let mut commands = BTreeMap::new();
        let mut consts = BTreeMap::new();
        let mut groups: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();

        for cmd in gl_cmd {
            if feature.commands.contains(&cmd.proto.name) {
                let key = cmd.proto.name.clone();
                let cmd = Command::new(cmd);

                for arg in cmd.args.iter().chain(std::iter::once(&cmd.decl)) {
                    for group in arg.group.iter() {
                        let type_ = arg.type_rust_raw();
                        let type_rust = match type_.as_str() {
                            "i32" => "u32",
                            s => s,
                        };
                        match groups.entry(group.to_owned()) {
                            btree_map::Entry::Occupied(o) => {
                                let (type_, _) = o.get();
                                assert_eq!(type_rust, type_.deref());
                            }
                            btree_map::Entry::Vacant(v) => {
                                v.insert((type_rust.to_owned(), Vec::new()));
                            }
                        }
                    }
                }

                commands.insert(key, cmd);
            }
        }

        for e in enums {
            if feature.enums.contains(&e.name) {
                for group in e.group.iter() {
                    if let Some((_, entry)) = groups.get_mut(group) {
                        entry.push(e.name.to_owned())
                    }
                }

                let key = e.name.to_owned();
                let con = Const::new(e);
                consts.insert(key, con);
            }
        }

        Self {
            commands,
            consts,
            groups,
            path: path
                .map(|p| p.as_ref().to_path_buf())
                .unwrap_or(PathBuf::from(".")),
        }
    }

    pub fn write(&self) -> Result<()> {
        create_dir_all(&self.path)?;

        self.write_file("consts.rs", gen_const)?;
        self.write_file("cmd.rs", gen_cmd)?;
        self.write_file("enums.rs", gen_enum)?;
        self.write_file("load.rs", gen_load)?;
        self.write_file("mod.rs", gen_mod)?;

        Ok(())
    }

    pub fn write_file<P, F>(&self, path: P, cb: F) -> Result<()>
    where
        P: AsRef<Path>,
        F: Fn(&Self, &mut dyn Write) -> Result<()>,
    {
        let path = self.path.join(path);
        let mut file = File::create(path)?;
        cb(self, &mut file)
    }

    pub fn has_group(&self, name: &str) -> bool {
        !self.groups[name].1.is_empty()
    }
}
