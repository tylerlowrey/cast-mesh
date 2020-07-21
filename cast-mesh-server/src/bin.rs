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

pub mod counter_service {
    tonic::include_proto!("counter");
}

#[derive(Default)]
pub struct CounterServiceProvider {
    pub db: DbConnection,
}

#[tonic::async_trait]
impl CounterService for CounterServiceProvider {
    async fn send(&self, request: Request<CountMessage>) -> Result<Response<CountResponse>, Status> {
        if let Ok(conn) = self.db.lock() {
            let data = request.into_inner();
            conn.execute(
                "INSERT INTO device_data(data) VALUES (?1, ?2)",
                       params![request.into_inner().count]
            );
            let reply = counter_service::ResponseCode::Ack;
            Ok(reply)
        }
        let reply = counter_service::ResponseCode::Reject;
        Ok(reply)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50055".parse().unwrap();
    let mut counter_service_provider = CounterServiceProvider::default();

    //Generate table for devices
    let conn = Connection::open(cast_mesh_server::SQLITE_DB_PATH)?;

    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS devices(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            address TEXT NOT NULL,
            port UNSIGNED INTEGER NOT NULL
        )"#,
        params![],
    )
    .expect("Unable to create devices table");

    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS device_data(
            id INTEGER PRIMARY KEY,
            data INTEGER NOT NULL
        )"#,
        params![]
    );

    let db_conn: DbConnection = Arc::new(Mutex::new(conn));

    counter_service_provider.db = db_conn.clone();

    let rocket_db_conn = db_conn.clone();

    let rocket_server_task = tokio::spawn(async {
        let db_conn = rocket_db_conn;
        let config = Config::build(Environment::Development)
            .address("127.0.0.1")
            .port(ROCKET_SERVER_PORT)
            .finalize()
            .expect("Unable to configure rocket server");

        let rocket_server = rocket::custom(config).attach(CORS());
        rocket_server
            .manage(db_conn)
            .mount(
                "/",
                routes![
                    routes::index,
                    routes::devices_options,
                    routes::device_options,
                    routes::register_device,
                    routes::get_devices_list,
                    routes::get_device_data,
                    routes::remove_device,
                ],
            )
            .launch();
    });

    let grpc_server_task = tokio::spawn(async {
            Server::builder().add_service(CounterService::new())
        });

    rocket_server_task
        .await
        .expect("Rocket server shutdown unexpectedly.");

    Ok(())
}
