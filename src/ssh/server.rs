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
    match sess.userauth_pubkey_file(&config.ssh.username, None, Path::new(&config.ssh.key), None) {
        Ok(_) => {
            if sess.authenticated() {
                info!("Authenticated: ssh to {}", config.ssh.address);

                // @TODO
                // command option to clean up
                match destroy {
                    Some(destroy) if destroy => {
                        let docker_compose_down = command(&sess, "sudo docker compose down -v");
                        println!("{}", docker_compose_down.trim());

                        let docker_system_prune = command(&sess, "sudo docker system prune -f");
                        println!("{}", docker_system_prune)
                    }
                    _ => {
                        // @NOTE
                        // Check docker exists or not.
                        let docker_version = command(&sess, "docker --version");
                        println!("{}", docker_version);

                        // @NOTE
                        // Install docker and docker-compose
                        if !docker_version.contains("Docker version") {
                            let install_docker = command(&sess, INSTALL_DOCKER);
                            println!("{}", install_docker.trim())
                        }

                        // @NOTE
                        // Upload file from local to the server.
                        scp(
                            &sess,
                            config.docker.docker_compose.as_str(),
                            config.docker.docker_compose.as_str(),
                        );

                        // @NOTE
                        // execute docker compose up
                        let docker_compose_up = command(&sess, "sudo docker compose up -d");
                        println!("{}", docker_compose_up);

                        // @NOTE
                        // execute docker compose up
                        let docker_ps = command(&sess, "sudo docker ps");
                        println!("{}", docker_ps)
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
