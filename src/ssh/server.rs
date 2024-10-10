use crate::{
    config,
    ssh::{command::command, scp::scp, ubuntu::INSTALL_DOCKER},
};
use log::{error, info, warn};
use ssh2::Session;
use std::{net::TcpStream, path::Path};

pub fn server(config: &config::Config, destroy: Option<bool>) {
    let tcp = TcpStream::connect(format!("{}:22", config.ssh.address)).unwrap();

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let public_key = sess.userauth_pubkey_file(
        &config.ssh.username,
        Some(Path::new("/Users/adiatma/.ssh/id_ed25519.pub")),
        Path::new(&config.ssh.key),
        None,
    );

    match public_key {
        Ok(_) => {
            if sess.authenticated() {
                info!("Authenticated: ssh to [_]@{}", config.ssh.address);

                match destroy {
                    Some(destroy) if destroy => {
                        let command_docker_compose_down = format!(
                            "sudo docker compose -f {} down -v",
                            config.docker.docker_compose
                        );
                        let docker_compose_down =
                            command(&sess, command_docker_compose_down.as_str());
                        println!("{}", docker_compose_down.trim());

                        let docker_system_prune = command(&sess, "sudo docker system prune -f");
                        println!("{}", docker_system_prune);

                        let docker_ps = command(&sess, "sudo docker ps");
                        println!("{}", docker_ps);

                        let docker_image_list = command(&sess, "sudo docker images");
                        println!("{}", docker_image_list)
                    }
                    _ => {
                        let docker_version = command(&sess, "docker --version");
                        println!("{}", docker_version);

                        if !docker_version.contains("Docker version") {
                            let install_docker = command(&sess, INSTALL_DOCKER);
                            println!("{}", install_docker.trim())
                        }

                        scp(
                            &sess,
                            config.docker.docker_compose.as_str(),
                            config.docker.docker_compose.as_str(),
                        );

                        let command_docker_compose_up = format!(
                            "sudo docker compose -f {} up -d",
                            config.docker.docker_compose
                        );
                        let docker_compose_up = command(&sess, command_docker_compose_up.as_str());
                        println!("{}", docker_compose_up);

                        let docker_ps = command(&sess, "sudo docker ps");
                        println!("{}", docker_ps);
                    }
                }
            } else {
                warn!("Unauthenticated: {}", config.ssh.address);
            }
        }
        Err(e) => {
            error!("{}", e.message())
        }
    }
}
