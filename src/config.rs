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

fn default_timeout() -> u64 {
    3
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
        #[serde(default = "default_timeout")]
        timeout: u64,
    },
    Binary {
        name: String,
        executable: String,
        #[serde(default)]
        args: Vec<String>,
    },
}
