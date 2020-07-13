mod pingpong;

use std::net::SocketAddr;
use tonic::transport::Server;

pub mod ping_pong {
    tonic::include_proto!("pingpong");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();

    println!("PingPong server listening on {}", addr);

    /*
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
     */

    Ok(())
}
