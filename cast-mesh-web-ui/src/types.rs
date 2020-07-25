use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub type DbConnection = Arc<Mutex<Connection>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceRegistration {
    pub device_name: String,
    pub device_address: String,
    pub device_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
    pub device_id: i32,
    pub device_name: String,
    pub device_address: String,
    pub device_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub error_msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceData {
    pub timestamp: i64,
    pub count: i64,
}

pub struct RegistrationResponse {
    response_code: i32,
    response_message: String,
}
