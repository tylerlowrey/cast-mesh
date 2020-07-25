#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use cast_mesh_server::cors::CORS;
use cast_mesh_server::types::DbConnection;
use cast_mesh_server::{routes, GRPC_SERVER_PORT, ROCKET_SERVER_PORT};
use rocket::config::Environment;
use rocket::Config;
use rusqlite::{params, Connection};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use crate::counter_service::{CountMessage, CountResponse};
use crate::counter_service::counter_service_server::{CounterService, CounterServiceServer};
use std::{thread, time};

pub mod counter_service {
    tonic::include_proto!("counter");
}

#[derive(Default)]
pub struct CounterServiceProvider {
    pub db: Option<DbConnection>,
}

#[tonic::async_trait]
impl CounterService for CounterServiceProvider {
    async fn send(&self,
                  request: Request<CountMessage>)
        -> Result<Response<CountResponse>, Status> {

    }
}

pub const SQLITE_DB_PATH: &str = "./data.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50055".parse().unwrap();
    let mut counter_service_provider = CounterServiceProvider::default();

    //Generate table for devices
    let conn = Connection::open(SQLITE_DB_PATH)?;

    let grpc_server_handle = thread::spawn(move || {
        let count = 0;
        let five_thousand_millis = time::Duration::from_millis(5000);
        loop {
            thread::sleep(five_thousand_millis);
            if let Ok(conn) = conn.lock() {
            }
        }
    });


    grpc_server_handle
        .await
        .expect("GRPC server shutdown unexpectedly.");

    Ok(())
}
