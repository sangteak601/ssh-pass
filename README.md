# ssh-pass

> [!WARNING]  
 This is not a secure way to manage SSH credentials or perform SSH authentication. Using SSH keys is always preferred for authentication, security, and auditability. You are solely responsible for any risks or damages that result from using this tool.

ssh-pass is a command-line tool written in Rust that simplifies SSH login using passwords, including support for jump hosts. It reads host credentials from a YAML configuration file and automatically handles password prompts verifications.

## Features

- Connect to SSH host with password-based authentication.
- Supports SSH jump hosts.
- Auto-accepts unknown host keys on first connection.
- Loads credentials from a simple YAML config file.

## Usage

```bash
ssh-pass [JUMP_HOST ...] <TARGET_HOST>
```

The last host is considered the main target, while all preceding hosts are treated as jump hosts.

### Example:
```
ssh-pass server internal-server
```

This will connect through server to reach internal-server.

## Configuration

By default, ssh-pass looks for a YAML configuration file at:

``` bash
$HOME/.ssh_pass.yaml
```

Or you can override the path with the environment variable:

```bash
export SSH_PASS_CONFIG_PATH=/path/to/custom_config.yaml
```

### Example Configuration (.ssh_pass.yaml)

```yaml
- host: server
  host_name: server.example.com
  port: 22
  user: user1
  password: mypassword

- host: internal-server
  host_name: 10.0.0.10
  user: user2
  password: password
```

Each entry includes:
- host: Identifier used on the command line.
- host_name: IP or domain of the host.
- port: (Optional) SSH port (default: 22).
- user: SSH username.
- password: SSH password.

## Installation

### Download Prebuilt .deb Package

1. Go to the [Releases](https://github.com/your-username/ssh-pass/releases) Page.
2. Download the latest .deb package.
3. Install it using dpkg:  
```bash
    sudo dpkg -i ssh-pass_x.x.x_amd64.deb
```

### Build from Source

Prerequisites
- Rust (latest stable)
- cargo build tool

1. Clone and Build  
```bash
git clone https://github.com/your-username/ssh-pass.git
cd ssh-pass
cargo build --release
```

2. Copy Files  
```bash
sudo cp target/release/ssh-pass /usr/local/bin/
sudo cp install/bash_completion /etc/bash_completion.d/ssh_pass_completion
```

## Limitations

- Does not support key-based authentication.
- Does not handle error case very well.
- Passwords may occasionally be printed to the terminal.

## Contribution

Contributions are welcome and appreciated!

- Found a bug? Please open an issue.
- Have a fix or improvement? Open a pull request.

## License

MIT License
