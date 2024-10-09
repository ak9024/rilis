# Rilis

Rilis is a tool for deployment built with Rust.

## Features

- Deploy effortlessly to VM (Virtual Machine)
- Just need `rilis.toml` for configuration.

## Installation

```shell
cargo install rilis
```

## Configuration

For configuration we named `rilis.toml`.

```toml
[ssh]
address = ""
username = "root"
key = "/Path/to/.ssh/id_rsa"

[docker]
compose = "docker-compose.yaml"
```

## License

MIT & Apache 2.0
