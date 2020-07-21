#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod cors;
mod registration;
use crate::cors::CORS;
use rocket::{config::Environment, Config};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

mod routes;

pub mod counter_service {
    tonic::include_proto!("counter");
}



const ROCKET_SERVER_PORT: u16 = 50060;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let counter_service_provider = CounterServiceProvider::default();

    let server_addr = Arc::new(Mutex::new(String::new()));

    let server_addr_cpy = server_addr.clone();

    let rocket_server_task = tokio::spawn(async {
        let server_addr = server_addr_cpy;
        let config = Config::build(Environment::Development)
            .address("0.0.0.0")
            .port(ROCKET_SERVER_PORT)
            .finalize()
            .expect("Unable to configure rocket server");

        let rocket_server = rocket::custom(config).attach(CORS());
        rocket_server
            .manage(server_addr)
            .mount("/", routes![routes::register_server,])
            .launch();
    });

    rocket_server_task
        .await
        .expect("Rocket server shutdown unexpectedly.");

    /*
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
     */

    Ok(())
}
