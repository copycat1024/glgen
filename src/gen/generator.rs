use super::{gen_cmd, gen_load, gen_mod};
use crate::{ffi::Command, gl::Registry};
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::{Result, Write},
    path::{Path, PathBuf},
};

pub struct Generator {
    pub commands: BTreeMap<String, Command>,
    path: PathBuf,
}

impl Generator {
    pub fn collect<P: AsRef<Path>>(registry: Registry, path: Option<P>) -> Self {
        let Registry {
            feature,
            commands: r_cmd,
            ..
        } = registry;
        let mut commands = BTreeMap::new();

        for cmd in r_cmd {
            if feature.commands.contains(&cmd.proto.name) {
                let key = cmd.proto.name.clone();
                let cmd = Command::new(cmd);
                commands.insert(key, cmd);
            }
        }

        Self {
            commands,
            path: path
                .map(|p| p.as_ref().to_path_buf())
                .unwrap_or(PathBuf::from(".")),
        }
    }

    pub fn write(&self) -> Result<()> {
        create_dir_all(&self.path)?;

        self.write_file("mod.rs", gen_mod)?;
        self.write_file("cmd.rs", gen_cmd)?;
        self.write_file("load.rs", gen_load)?;

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
}
