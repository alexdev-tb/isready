use crate::config::*;
use port_check::*;
use std::{
    io::{self, Write},
    process::Command,
    time::Duration,
};

fn check_binary(executable: &str, args: &[String], output: bool) -> bool {
    let result = Command::new(executable).args(args).output();
    match result {
        Ok(out) => {
            if output {
                io::stdout().write_all(&out.stdout).unwrap();
                io::stderr().write_all(&out.stderr).unwrap();
            }
            out.status.success()
        }
        Err(_) => false,
    }
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
                executable,
                args,
                output,
                ..
            } => check_binary(executable, args, *output),
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
