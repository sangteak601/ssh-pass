use std::{collections::HashMap, env, fs};
use clap::Parser;
use serde::{Serialize, Deserialize};

const DEFAULT_CONFIG_FILE: &str = ".ssh_pass.yaml";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(required = true, help = "Host(s) to connect to. The last host is the main target and the others are jump hosts.")]
    pub hosts: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub host_name: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
}

pub fn load_config() -> Result<HashMap<String, Config>, Box<dyn std::error::Error>> {
    let config_path = env::var("SSH_PASS_CONFIG_PATH").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| ".".into());
        format!("{}/{}", home, DEFAULT_CONFIG_FILE)
    });

    let configs: Vec<Config> = serde_yaml::from_str(&fs::read_to_string(config_path)?)?;

    let mut config_map: HashMap<String, Config> = HashMap::new();

    for config in configs {
        config_map.insert(config.host.clone(), config);
    }

    if config_map.is_empty() {
        return Err(Box::from("No configurations found in the config file."));
    }

    Ok(config_map)
}
