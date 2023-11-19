use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;

#[derive(Clone, Copy)]
pub struct Options {
    configure: bool,
    build: bool,
    dependencies: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            configure: false,
            build: false,
            dependencies: false,
        }
    }

    pub fn configure(&mut self, value: bool) -> Options {
        self.configure = value;

        return *self;
    }

    pub fn build(&mut self, value: bool) -> Options {
        self.build = value;

        return *self;
    }

    pub fn dependencies(&mut self, value: bool) -> Options {
        self.dependencies = value;

        return *self;
    }

    pub fn install(&self, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
        let build = format!("{repo}/BUILD");
        let script = format!("{repo}/BUILD.sh");

        fs::copy(&build, &script)?;

        let mut fd = File::options()
            .append(true)
            .open(&script)?;

        if self.configure {
            fd.write_all(b"\nconfig\n")?;
        }

        if self.build {
            fd.write_all(b"\nbuild\n")?;
        }

        if self.dependencies{
            fd.write_all(b"\ndependencies\n")?;
        }

        let child = Command::new("sh")
            .args([&script])
            .output()?;

        fs::remove_file(&script)?;

        if child.status.success() {
            Ok(())
        } else {
            Err("exited with non-zero exit code".into())
        }
    }
}

