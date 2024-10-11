# Rilis

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ak9024/rilis/ci.yml?style=for-the-badge) ![GitHub License](https://img.shields.io/github/license/ak9024/rilis?style=for-the-badge) ![GitHub repo size](https://img.shields.io/github/repo-size/ak9024/rilis?style=for-the-badge)

[![asciicast](https://asciinema.org/a/680195.svg)](https://asciinema.org/a/680195)

## 🚧 Notes

> This project still under development

## Key Features

- Effortless VM (Virtual Machine) Deployment
- Simply configure with a file `deploy.toml` or `anything.toml` and use `docker-compose.yaml` locally.


## Installation

```shell
cargo install --git https://github.com/ak9024/rilis
```

## Configuration

Create configuration named `deploy.toml`.

```toml
[ssh]
# IPv4 address can be define here.
address = ""
username = ""
# You can choose to using "password" or "private_key"
password = ""
# Your private_key location
private_key = "/Users/adiatma/.ssh/id_ed25519"
# by default port 22, but you can customize by your self.
port = 22

[server]
# scp is optional, if do you want to copying to file from local to the server, can be define here.
scp = [ "docker-compose.yaml" ]
# ssh commands if do you want to exec script or anything on the server.
commands = [
  "sudo docker compose -f docker-compose.yaml up -d",
  "sudo docker ps",
]
```

> You can define multiple configuration for multiple use case for automation, examples to provision VM using `setup.toml`, then for deployment using `deploy.toml`.

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

Then start `rilis --cfg deploy.toml`

## More

```shell
rilis -h
```

## 🚧 Next

- [x] Setup `docker` and `docker compose` on the server.
- [x] Able to `scp` connection.
- [x] Able to customize configuration via `rilis.toml`
- [ ] Setup `CI/CD`

## License

MIT & Apache 2.0
