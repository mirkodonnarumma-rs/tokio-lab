use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server in ascolto su :8080");

    loop {
        // accept() restituisce (TcpStream, SocketAddr)
        // Il TcpStream è owned — viene mosso nel blocco sotto
        let (mut socket, addr) = listener.accept().await?;
        println!("Connessione da {addr}");

        // Gestione sequenziale (blocca l'accept di nuovi client!)
        let mut buf = [0u8; 1024];
        loop {
            let n = socket.read(&mut buf).await?;
            if n == 0 { break; } // client ha chiuso la connessione
            socket.write_all(&buf[..n]).await?;
        }
        println!("Client {addr} disconnesso");
    }
}