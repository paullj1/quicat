
use s2n_quic::Server;
use std::error::Error;

pub async fn _server(cert: &str, key: &str, address: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let addr: String = format!("{}:{}",address,port);
    let mut server = Server::builder()
        .with_tls((cert, key))?
        .with_io(addr.as_str())?
        .start()?;

    while let Some(mut connection) = server.accept().await {
        eprintln!("Connection accepted from {:?}", connection.remote_addr());

        while let Ok(Some(stream)) = connection.accept_bidirectional_stream().await {
            let (mut receive_stream, mut send_stream) = stream.split();

            // spawn a task that copies responses from the server to stdout
            tokio::spawn(async move {
                let mut stdout = tokio::io::stdout();
                let _ = tokio::io::copy(&mut receive_stream, &mut stdout)
                    .await;
            });

            // copy data from stdin and send it to the server
            let mut stdin = tokio::io::stdin();
            tokio::io::copy(&mut stdin, &mut send_stream)
                .await
                .expect("Connection closed");
        }
        eprintln!("Stream closed")
    }
    Ok(())
}

