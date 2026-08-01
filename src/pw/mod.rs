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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn port_forward_errors_on_invalid_bind_address() {
        // Not a valid `host:port` socket address, so binding must fail.
        let result = port_forward("not-a-valid-address", "127.0.0.1:9").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn handle_connection_proxies_bytes_bidirectionally() {
        // Start an "upstream" echo-ish server that returns a fixed reply.
        let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let remote_addr = upstream.local_addr().unwrap().to_string();

        let upstream_task = tokio::spawn(async move {
            let (mut socket, _) = upstream.accept().await.unwrap();
            let mut buf = [0u8; 5];
            socket.read_exact(&mut buf).await.unwrap();
            assert_eq!(&buf, b"hello");
            socket.write_all(b"world").await.unwrap();
        });

        // Accept one inbound connection and forward it to the upstream.
        let proxy = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let proxy_addr = proxy.local_addr().unwrap();
        let proxy_task = tokio::spawn(async move {
            let (inbound, _) = proxy.accept().await.unwrap();
            handle_connection(inbound, remote_addr).await.unwrap();
        });

        // Client speaks to the proxy and should transparently reach upstream.
        let mut client = TcpStream::connect(proxy_addr).await.unwrap();
        client.write_all(b"hello").await.unwrap();
        let mut reply = [0u8; 5];
        client.read_exact(&mut reply).await.unwrap();
        assert_eq!(&reply, b"world");

        // Drop the client so the proxy sees EOF on the inbound half; otherwise
        // `handle_connection`'s bidirectional join would never complete.
        drop(client);

        upstream_task.await.unwrap();
        proxy_task.await.unwrap();
    }
}
