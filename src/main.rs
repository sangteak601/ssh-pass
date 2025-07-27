use clap::Parser;
use expectrl::{Expect, spawn, stream::stdin::Stdin};
use std::{
    collections::HashMap,
    io::{Write, stdout},
    time::Duration,
};

use ssh_pass::{Cli, Config, load_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    let hosts = args.hosts;

    let config = match load_config() {
        Ok(config) => config,
        Err(err) => {
            return Err(Box::from(format!("Unable to load config: {}", err)));
        }
    };

    validate_hosts(&hosts, &config)?;

    let mut stdin = Stdin::open().expect("Failed to create stdin");

    let result = run(&hosts, &config, &mut stdin);

    stdin.close().expect("Failed to close a stdin");

    match &result {
        Ok(_) => println!("SSH session ended successfully. See you next time! 👋"),
        Err(err) => return Err(Box::from(format!("Error during SSH session: {}", err))),
    }

    Ok(())
}

fn validate_hosts(hosts: &Vec<String>, config: &HashMap<String, Config>) -> Result<(), String> {
    for host in hosts {
        if !config.contains_key(host) {
            return Err(format!("No configuration found for host '{}'", host));
        }
    }
    Ok(())
}

fn run(
    hosts: &Vec<String>,
    config: &HashMap<String, Config>,
    stdin: &mut Stdin,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command: String = "ssh ".into();
    if hosts.len() > 1 {
        command.push_str(" -J");
    }

    for (i, host) in hosts.iter().enumerate() {
        if let Some(cfg) = config.get(host) {
            if i == 0 || i == hosts.len() - 1 {
                command.push_str(&format!(" {}@{}", cfg.user, cfg.host_name));
            } else {
                command.push_str(&format!(",{}@{}", cfg.user, cfg.host_name));
            }
        }
    }

    let mut session = spawn(&command).map_err(|_| -> Box<dyn std::error::Error> {
        Box::from(format!(
            "Failed to spawn SSH session with command: {}",
            command
        ))
    })?;
    session.set_expect_timeout(Some(Duration::from_secs(1)));

    for host in hosts {
        if let Some(cfg) = config.get(host) {
            if let Ok(_) = session.expect("Are you sure you want to continue connecting") {
                session.send_line("yes")?;
                session.flush()?;
            }
            if let Ok(_) = session.expect("password:") {
                session.send_line(&cfg.password)?;
                session.flush()?;
            }
        }
    }

    println!("\nNow you're in interactive terminal. To exit, type 'exit' or 'CTRL + D'.");

    session
        .interact(stdin, stdout())
        .spawn()
        .expect("Failed to start interact");

    Ok(())
}
