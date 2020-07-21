use serde::{Serialize, Deserialize};
use rusqlite::Connection;
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

pub struct RegistrationResponse {
    response_code: i32,
    response_message: String,
}
