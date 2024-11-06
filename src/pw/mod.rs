use log::{error, info};
use tokio::io::{self};
use tokio::net::{TcpListener, TcpStream};

pub async fn handle_connection(inbound: TcpStream, remote_addr: String) -> io::Result<()> {
    let outbound = TcpStream::connect(remote_addr).await?;

    let (mut ri, mut wi) = tokio::io::split(inbound);
    let (mut ro, mut wo) = tokio::io::split(outbound);

    let client_to_server = tokio::io::copy(&mut ri, &mut wo);
    let server_to_client = tokio::io::copy(&mut ro, &mut wi);

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

pub async fn port_forward(local_addr: &str, remote_addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(local_addr).await?;

    info!("Listening on: {}", local_addr);

    loop {
        let (inbound, _) = listener.accept().await?;

        let remote_addr = remote_addr.to_owned();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(inbound, remote_addr).await {
                error!("{}", e);
            }
        });
    }
}
