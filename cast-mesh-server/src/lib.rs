#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

pub mod cors;

pub mod routes {
    use rocket_contrib::json::{Json, JsonValue};
    use crate::types::DeviceInfo;

    #[get("/")]
    pub fn index() -> &'static str {
        "Hello, World!"
    }

    #[post("/devices/register", format = "json", data = "<body>")]
    pub fn register_device(body: Json<DeviceInfo>) {

        let request = body.into_inner();
        println!("Received registration request: [{}, {}, {}] ",
                 request.device_name, request.device_address, request.device_port);
    }

    #[delete("/devices/<device_id>")]
    pub fn remove_device(device_id: i32) {
        println!("Device with id {} removed", device_id);
    }

    #[get("/devices")]
    pub fn get_devices_list() -> Json<Vec<DeviceInfo>> {
        let devices = vec![
            DeviceInfo {
                device_name: String::from("Smart Scale"),
                device_address: String::from("127.0.0.1"),
                device_port: 9000
            },
            DeviceInfo {
                device_name: String::from("Smart Fridge"),
                device_address: String::from("127.0.0.1"),
                device_port: 9001
            },
        ];

        Json(devices)
    }

    #[get("/devices/<device_id>")]
    pub fn get_device_data(device_id: i32) -> JsonValue {
        json!({
            "data": [
                { "timestamp": 1100, "value": true },
                { "timestamp": 1101, "value": true },
                { "timestamp": 1102, "value": false }
            ]
        })
    }
}

pub mod types {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DeviceInfo {
        pub device_name: String,
        pub device_address: String,
        pub device_port: u16,
    }

    pub struct RegistrationResponse {
        response_code: i32,
        response_message: String,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
