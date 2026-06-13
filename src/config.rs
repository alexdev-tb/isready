use std::time::Duration;

use port_check::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub project: Project,
    pub check: Vec<Check>,
}
#[derive(Deserialize)]
pub struct Project {
    pub name: String,
}

pub trait Checkable {
    fn run(&self) -> bool;
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Check {
    Env {
        name: String,
        var: String,
    },
    Tcp {
        name: String,
        host: String,
        port: u16,
    },
    Binary {
        name: String,
        executable: String,
    },
}

impl Checkable for Check {
    fn run(&self) -> bool {
        match self {
            Check::Tcp { host, port, .. } => {
                let address = format!("{}:{}", host, port);
                is_port_reachable_with_timeout(address, Duration::from_secs(5))
            }
            Check::Env { var, .. } => std::env::var(var).is_ok(),
            Check::Binary { executable, .. } => which::which(executable).is_ok(),
        }
    }
}
impl Check {
    pub fn name(&self) -> &str {
        match self {
            Check::Tcp { name, .. } => name,
            Check::Env { name, .. } => name,
            Check::Binary { name, .. } => name,
        }
    }
}
