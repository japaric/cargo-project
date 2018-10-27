//! Library to retrieve information about a Cargo project
//!
//! Useful for building Cargo subcommands that require building the project.

#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate failure;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate rustc_cfg;

mod config;
mod manifest;
mod workspace;

use std::{
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use rustc_cfg::Cfg;
use serde::Deserialize;
use toml::de;

use config::Config;
use manifest::Manifest;

/// Information about a Cargo project
pub struct Project {
    name: String,

    target: Option<String>,

    target_dir: PathBuf,
}

/// Errors
#[derive(Debug, Fail)]
pub enum Error {
    /// error: not a Cargo project
    #[fail(display = "not a Cargo project")]
    NotACargoProject,
}

impl Project {
    /// Retrieves information about the Cargo project at the given `path`
    ///
    /// `path` doesn't need to be the directory that contains the `Cargo.toml` file; it can be any
    /// point within the Cargo project.
    pub fn query<P>(path: P) -> Result<Self, failure::Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let root = search(path, "Cargo.toml").ok_or(Error::NotACargoProject)?;

        // parse Cargo.toml
        let manifest = parse::<Manifest>(&root.join("Cargo.toml"))?;

        // parse .cargo/config
        let mut target = None;
        let mut target_dir = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from);
        if let Some(path) = search(root, ".cargo/config") {
            let config: Config = parse(&path.join(".cargo/config"))?;

            if let Some(build) = config.build {
                target = build.target;
                target_dir = target_dir.or(build.target_dir.map(PathBuf::from));
            }
        }

        // is this project member of a workspace?
        let mut cwd = root.parent();
        let mut workspace = None;
        while let Some(path) = cwd {
            if let Some(outer_root) = search(path, "Cargo.toml") {
                if let Ok(manifest) = parse::<workspace::Manifest>(&outer_root.join("Cargo.toml")) {
                    // this is indeed a workspace
                    if manifest
                        .workspace
                        .members
                        .iter()
                        .any(|member| outer_root.join(member) == root)
                    {
                        // we are a member of this workspace
                        workspace = Some(outer_root);
                        break;
                    }
                }

                // this is not a workspace; keep looking
                cwd = outer_root.parent();
                continue;
            }

            break;
        }

        target_dir = target_dir.or_else(|| workspace.map(|path| path.join("target")));

        Ok(Project {
            name: manifest.package.name,
            target,
            target_dir: target_dir.unwrap_or(root.join("target")),
        })
    }

    /// Returns the path to a build artifact
    ///
    /// # Inputs
    ///
    /// - `artifact` is the kind of build artifact: `Bin` (`--bin`), `Example` (`--example`), `Lib`
    /// (`--lib`)
    /// - `profile` is the compilation profile: `Dev` or `Release` (`--release`)
    /// - `target` is the specified compilation target (`--target`)
    /// - `host` is the triple of host -- this is used as the compilation target when no `target` is
    /// specified and the project has no default build target
    pub fn path(
        &self,
        artifact: Artifact,
        profile: Profile,
        target: Option<&str>,
        host: &str,
    ) -> Result<PathBuf, failure::Error> {
        let mut path = self.target_dir().to_owned();

        if let Some(target) = target.or(self.target()) {
            path.push(target);
        }

        let cfg = Cfg::of(target.or(self.target()).unwrap_or(host))?;

        match profile {
            Profile::Dev => path.push("debug"),
            Profile::Release => path.push("release"),
            Profile::__HIDDEN__ => unreachable!(),
        }

        match artifact {
            Artifact::Bin(bin) => {
                path.push(bin);

                if cfg.target_arch == "wasm32" {
                    path.set_extension("wasm");
                } else if cfg
                    .target_family
                    .as_ref()
                    .map(|f| f == "windows")
                    .unwrap_or(false)
                {
                    path.set_extension("exe");
                }
            }
            Artifact::Example(example) => {
                path.push("examples");
                path.push(example);

                if cfg.target_arch == "wasm32" {
                    path.set_extension("wasm");
                } else if cfg
                    .target_family
                    .as_ref()
                    .map(|f| f == "windows")
                    .unwrap_or(false)
                {
                    path.set_extension("exe");
                }
            }
            Artifact::Lib => {
                path.push(format!("lib{}.rlib", self.name().replace("-", "_")));
            }
            Artifact::__HIDDEN__ => unreachable!(),
        }

        Ok(path)
    }

    /// Returns the name of the project (`package.name`)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the default compilation target
    pub fn target(&self) -> Option<&str> {
        self.target.as_ref().map(|s| &**s)
    }

    /// Returns the target directory path
    ///
    /// This is where build artifacts are placed
    pub fn target_dir(&self) -> &Path {
        &self.target_dir
    }
}

/// Build artifact
#[derive(Clone, Copy)]
pub enum Artifact<'a> {
    /// Binary (`--bin`)
    Bin(&'a str),
    /// Example (`--example`)
    Example(&'a str),
    /// Library (`--lib`)
    Lib,
    #[doc(hidden)]
    __HIDDEN__,
}

/// Build profile
#[derive(PartialEq)]
pub enum Profile {
    /// Development profile
    Dev,
    /// Release profile (`--release`)
    Release,
    #[doc(hidden)]
    __HIDDEN__,
}

impl Profile {
    /// Is this the release profile?
    pub fn is_release(&self) -> bool {
        *self == Profile::Release
    }
}

/// Search for `file` in `path` and its parent directories
fn search<'p>(mut path: &'p Path, file: &str) -> Option<&'p Path> {
    loop {
        if path.join(file).exists() {
            return Some(path);
        }

        if let Some(p) = path.parent() {
            path = p;
        } else {
            return None;
        }
    }
}

fn parse<T>(path: &Path) -> Result<T, failure::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(de::from_str(&s)?)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::{Artifact, Profile, Project};

    #[test]
    fn path() {
        let project = Project::query(env::current_dir().unwrap()).unwrap();

        let thumb = "thumbv7m-none-eabi";
        let wasm = "wasm32-unknown-unknown";
        let windows = "x86_64-pc-windows-msvc";
        let linux = "x86_64-unknown-linux-gnu";

        let p = project
            .path(Artifact::Bin("foo"), Profile::Dev, None, windows)
            .unwrap();

        assert!(p.to_str().unwrap().ends_with("target/debug/foo.exe"));

        let p = project
            .path(Artifact::Example("bar"), Profile::Dev, None, windows)
            .unwrap();

        assert!(p
            .to_str()
            .unwrap()
            .ends_with("target/debug/examples/bar.exe"));

        let p = project
            .path(Artifact::Bin("foo"), Profile::Dev, Some(thumb), windows)
            .unwrap();

        assert!(p
            .to_str()
            .unwrap()
            .ends_with(&format!("target/{}/debug/foo", thumb)));

        let p = project
            .path(Artifact::Example("bar"), Profile::Dev, Some(thumb), windows)
            .unwrap();

        assert!(p
            .to_str()
            .unwrap()
            .ends_with(&format!("target/{}/debug/examples/bar", thumb)));

        let p = project
            .path(Artifact::Bin("foo"), Profile::Dev, Some(wasm), linux)
            .unwrap();

        assert!(p
            .to_str()
            .unwrap()
            .ends_with(&format!("target/{}/debug/foo.wasm", wasm)));

        let p = project
            .path(Artifact::Example("bar"), Profile::Dev, Some(wasm), linux)
            .unwrap();

        assert!(p
            .to_str()
            .unwrap()
            .ends_with(&format!("target/{}/debug/examples/bar.wasm", wasm)));
    }
}
