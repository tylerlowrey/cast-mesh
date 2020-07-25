use crate::routes::types::ServerRegistration;
use rocket::State;
use rocket_contrib::json::Json;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

#[post("/register_server", format = "json", data = "<registration_json>")]
pub fn register_server(
    registration_json: Json<ServerRegistration>,
    remote_addr: SocketAddr,
    server_addr: State<Arc<Mutex<String>>>,
    start_send: State<Arc<AtomicBool>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let registration_msg = registration_json.into_inner();
    let server_addr_string = format!("{}:{}", remote_addr.ip(), &registration_msg.server_port);
    if let Ok(mut server_addr) = server_addr.lock() {
        *server_addr = server_addr_string.clone();
        start_send.store(true, Ordering::SeqCst);
    }

    Ok(format!(
        "Server {}:{} registered successfully",
        server_addr_string, registration_msg.server_port
    ))
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ServerRegistration {
        pub server_port: u16,
    }
}
