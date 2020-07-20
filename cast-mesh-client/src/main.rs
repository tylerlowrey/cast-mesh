mod registration;

use std::net::SocketAddr;
use tonic::transport::Server;

pub mod registration_service {
    tonic::include_proto!("registration");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();

    println!("Device server listening on {}", addr);

    /*
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
     */

    Ok(())
}
