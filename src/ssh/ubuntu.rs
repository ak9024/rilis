pub const INSTALL_DOCKER: &str = r#"
    sudo apt update;
    sudo apt install -y apt-transport-https ca-certificates curl software-properties-common;
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg;
    echo 'deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu focal stable' | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null;
    sudo apt update;
    sudo apt install -y docker-ce docker-ce-cli containerd.io;
    docker version;
"#;
