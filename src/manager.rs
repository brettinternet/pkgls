use crate::cmd::{pacman::PacmanCmd, ListInstalled};
use crate::error::*;
use std::boxed::Box;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::{env, fs};

#[derive(Debug, Clone, Copy)]
pub enum ManagerKind {
    Pacman,
}

impl Display for ManagerKind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Debug for dyn ListInstalled {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}

impl Display for dyn ListInstalled {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}

/// Convert string to kind enum
impl FromStr for ManagerKind {
    type Err = ();
    fn from_str(input: &str) -> std::result::Result<ManagerKind, Self::Err> {
        match input.to_lowercase().as_str() {
            "pacman" => Ok(ManagerKind::Pacman),
            _ => Err(()),
        }
    }
}

fn get_cmd(kind: ManagerKind) -> Box<dyn ListInstalled> {
    match kind {
        ManagerKind::Pacman => Box::new(PacmanCmd::new()),
    }
}

/// Check if program exists in the environment's $PATH
///
/// Source: https://stackoverflow.com/a/35046243
fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let formatted_path = format!("{}/{}", p, program);
            if fs::metadata(formatted_path).is_ok() {
                return true;
            }
        }
    }
    false
}

/// Determine manager based on common programs available in $PATH
fn get_manager_kind() -> Result<ManagerKind> {
    if is_program_in_path("pacman") {
        debug!("Detected {} package manager", "pacman");
        Ok(ManagerKind::Pacman)
    } else {
        Err(ErrorKind::UndetectedManager(env::consts::OS.to_string()).into())
    }
}

/// Package manager
#[derive(Debug)]
pub struct Manager {
    pub kind: ManagerKind,
    pub cmd: Box<dyn ListInstalled>,
}

impl Manager {
    pub fn new() -> Result<Self> {
        let kind = get_manager_kind()?;
        let mgr = Self {
            kind,
            cmd: get_cmd(kind),
        };
        Ok(mgr)
    }

    /// Set manager kind
    fn set_kind<'a>(&'a mut self, kind: ManagerKind) -> &'a mut Self {
        self.kind = kind;
        self.cmd = get_cmd(kind);
        self
    }

    /// Configure the manager kind from input
    pub fn configure_kind(&mut self, input: &str) -> Result<ManagerKind> {
        match ManagerKind::from_str(input) {
            Ok(kind) => {
                self.set_kind(kind);
                Ok(kind)
            }
            Err(_) => Err(ErrorKind::UnsupportedManager(
                input.to_string(),
                self.kind.to_string().to_lowercase(),
            )
            .into()),
        }
    }
}
