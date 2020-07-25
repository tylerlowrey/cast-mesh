#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod cors;
pub mod types;

pub const SQLITE_DB_PATH: &str = "./data.db";
pub const ROCKET_SERVER_PORT: u16 = 50050;
pub const GRPC_SERVER_PORT: u16 = 50055;

pub mod routes {
    use crate::types::{DbConnection, DeviceInfo, DeviceRegistration, DeviceData};
    use crate::{GRPC_SERVER_PORT, ROCKET_SERVER_PORT, SQLITE_DB_PATH};
    use rocket::State;
    use rocket_contrib::json::{Json, JsonValue};
    use rusqlite::{params, Connection, NO_PARAMS};
    use std::error::Error;

    #[get("/")]
    pub fn index() -> &'static str {
        "Hello, World!"
    }

    #[options("/devices")]
    pub fn devices_options() {}

    #[options("/devices/<device_id>", format = "json")]
    pub fn device_options(device_id: i32) {}

    #[post("/devices", format = "json", data = "<body>")]
    pub fn register_device(body: Json<DeviceRegistration>, db: State<DbConnection>) {
        if let Ok(conn) = db.lock() {
            let request = body.into_inner();

            /*
            let client = reqwest::blocking::Client::new();
            let res = client
                .post(&format!(
                    "http://{}:{}/register_server",
                    request.device_address, request.device_port
                ))
                .header("Content-Type", "application/json")
                .body(format!("{{\"server_port\":{}}}", GRPC_SERVER_PORT))
                .send()
                .unwrap();


             */

            conn.execute(
                "INSERT INTO devices(name, address, port) VALUES (?1, ?2, ?3)",
                params![
                    request.device_name,
                    request.device_address,
                    request.device_port
                ],
            );

            println!(
                "Received registration request: [{}, {}, {}] ",
                request.device_name, request.device_address, request.device_port
            );

        }
    }

    #[delete("/devices/<device_id>")]
    pub fn remove_device(device_id: i32, db: State<DbConnection>) {
        if let Ok(conn) = db.lock() {
            let mut stmt = conn
                .prepare("DELETE FROM devices WHERE id = ?1")
                .expect("Unable to create prepared statement for devices deletion");

            stmt.execute(params![device_id])
                .expect(&format!("Unable to delete device: {}", device_id));
        }
    }

    #[get("/devices")]
    pub fn get_devices_list(db: State<DbConnection>) -> Json<Vec<DeviceInfo>> {
        if let Ok(conn) = db.lock() {
            let mut stmt = conn
                .prepare("SELECT id, name, address, port FROM devices")
                .expect("Unable to create prepared statement for devices list");
            let devices_iter = stmt
                .query_map(NO_PARAMS, |row| {
                    Ok(DeviceInfo {
                        device_id: row.get(0)?,
                        device_name: row.get(1)?,
                        device_address: row.get(2)?,
                        device_port: row.get(3)?,
                    })
                })
                .expect("Error while retrieving devices list");

            let mut devices = Vec::new();

            for device in devices_iter {
                devices.push(device.unwrap());
            }
            Json(devices)
        } else {
            Json(Vec::new())
        }
    }

    #[get("/devices/<device_id>")]
    pub fn get_device_data(device_id: i32, db: State<DbConnection>) -> Json<Vec<DeviceData>>{
        /*
        if let Ok(conn) = db.lock() {
            let mut stmt = conn
                .prepare("SELECT timestamp, data port FROM devices")
                .expect("Unable to create prepared statement for devices list");
            let data_iter = stmt.query_map(NO_PARAMS, |row| {
                Ok(DeviceData{
                    timestamp: row.get(0)?,
                    count: row.get(1)?,
                })
            }).expect("Error while retrieving device data");
            let mut data = Vec::new();
            for data_point in data_iter {
                data.push(data_point.unwrap());
            }
            Json(data)
        } else {
            Json(Vec::new())
        }
         */
        Json(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
