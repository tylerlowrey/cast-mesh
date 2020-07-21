#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use cast_mesh_server::{routes, ROCKET_SERVER_PORT};
use rocket::Config;
use rocket::config::Environment;
use cast_mesh_server::cors::CORS;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use cast_mesh_server::types::DbConnection;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //Generate table for devices
    let conn = Connection::open(cast_mesh_server::SQLITE_DB_PATH)?;

    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS devices(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            address TEXT NOT NULL,
            port UNSIGNED INTEGER NOT NULL
        )"#, params![]
    ).expect("Unable to create devices table");

    let db_conn: DbConnection = Arc::new(Mutex::new(conn));

    let rocket_db_conn = db_conn.clone();

    let rocket_server_task = tokio::spawn(async {
        let db_conn = rocket_db_conn;
        let config = Config::build(Environment::Development)
            .address("127.0.0.1")
            .port(ROCKET_SERVER_PORT)
            .finalize()
            .expect("Unable to configure rocket server");

        let rocket_server = rocket::custom(config).attach(CORS());
        rocket_server.manage(db_conn)
            .mount("/", routes![
                routes::index,
                routes::devices_options,
                routes::device_options,
                routes::register_device,
                routes::get_devices_list,
                routes::get_device_data,
                routes::remove_device,
            ])
            .launch();
    });

    rocket_server_task.await.expect("Rocket server shutdown unexpectedly.");

    Ok(())
}