use crate::config::*;
use port_check::*;
use std::{process::Command, time::Duration};

fn check_binary(executable: &str, args: &[String]) -> bool {
    Command::new(executable).args(args).output().is_ok()
}

impl Checkable for Check {
    fn run(&self) -> bool {
        match self {
            Check::Tcp {
                host,
                port,
                timeout,
                ..
            } => {
                let address = format!("{}:{}", host, port);
                is_port_reachable_with_timeout(address, Duration::from_secs(*timeout))
            }
            Check::Env { var, .. } => std::env::var(var).is_ok(),
            Check::Binary {
                executable, args, ..
            } => check_binary(executable, args),
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
