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
use crate::counter_service::counter_service_client::CounterServiceClient;
use crate::counter_service::{CountMessage, CountResponse};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

mod routes;

pub mod counter_service {
    tonic::include_proto!("counter");
}



const ROCKET_SERVER_PORT: u16 = 50060;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let send_data = Arc::new(AtomicBool::new(false));
    let send_data_grpc = send_data.clone();

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
            .manage(send_data)
            .mount("/", routes![routes::register_server,])
            .launch();
    });

    let grpc_client_task = tokio::spawn(async move {
        let mut client;
        let mut current_count = 0;
        loop {
            if send_data_grpc.load(Ordering::SeqCst) {
                client = CounterServiceClient::connect(format!("http://{}", "")).await.expect("Could not connect");
                loop {
                    current_count += 1;
                    let request = tonic::Request::new(CountMessage {
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time error").as_secs() as i64,
                        count: current_count,
                    });
                    let _ = client.send(request).await.expect("oops");

                    tokio::time::delay_for(Duration::new(5, 0));
                }
            }
        }

    });

    rocket_server_task
        .await
        .expect("Rocket server shutdown unexpectedly.");

    Ok(())
}
