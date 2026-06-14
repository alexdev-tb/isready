use crate::config::*;
use port_check::*;
use std::time::Duration;

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
