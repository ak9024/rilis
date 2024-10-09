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
key = "/path/to/.ssh/id_rsa"

[docker]
compose = "docker-compose.yaml"
```

## Docker

Prepare your `docker-compose.yaml`

```yaml
version: "3.8"

services:
  test:
    image: httpd:latest
    ports:
      - "3000:80"
```

Then start `rilis`

## Server Support

- Ubuntu

## 🚧 TODO

- [x] Setup `docker`` and `docker compose` on the server.
- [x] Able to `scp` connection.
- [x] Able to customize configuration via `rilis.toml`
- [ ] Setup `CI/CD`
- [ ] Able to pull and build image on the server.

## License

MIT & Apache 2.0
