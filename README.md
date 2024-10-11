# Rilis

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ak9024/rilis/ci.yml?style=for-the-badge) ![GitHub License](https://img.shields.io/github/license/ak9024/rilis?style=for-the-badge) ![GitHub repo size](https://img.shields.io/github/repo-size/ak9024/rilis?style=for-the-badge)

[![asciicast](https://asciinema.org/a/680195.svg)](https://asciinema.org/a/680195)

## Why?

I build this tools for my use cases, i need to communicate with my server then need to automate deployment from my local computer to the VM server i have.

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
# required IPv4 address can be define here.
address = ""
# optional, by default is "root"
username = ""
# You can choose one using "password" or "private_key"
password = ""
# optional if do you want to connect via private_key
private_key = "/Users/adiatma/.ssh/id_ed25519"
# optional by default port 22, but you can customize by your self.
port = 22

[server]
# scp is optional, if do you want to copy file from local to the server, can be define here.
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
  home:
    container_name: home
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
- [x] Setup `CI`
- [ ] Setup `CD` realesed to crate.

## License

MIT & Apache 2.0
