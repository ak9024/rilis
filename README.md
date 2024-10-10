# Rilis

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ak9024/rilis/ci.yml?style=for-the-badge)

[![asciicast](https://asciinema.org/a/680195.svg)](https://asciinema.org/a/680195)

## 🚧 Notes

> This CLI is under development

## Features

- Effortless VM (Virtual Machine) Deployment
- Simply configure with a `rilis.toml` and use `docker-compose.yaml` locally.


## Installation

```shell
cargo install --git https://github.com/ak9024/rilis
```

## Configuration

Create configuration named `rilis.toml`.

```toml
[ssh]
# your public IPv4 address
address = ""
# default "root"
username = ""
# your private key ~/.ssh/id_rsa
private_key = "/path/to/.ssh/id_rsa"

[docker]
# your docker compose location
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

## More

```shell
rilis -h
```

## Server Support

- Ubuntu

## 🚧 TODO

- [x] Setup `docker` and `docker compose` on the server.
- [x] Able to `scp` connection.
- [x] Able to customize configuration via `rilis.toml`
- [ ] Setup `CI/CD`
- [ ] Able to pull and build image on the server.

## License

MIT & Apache 2.0
