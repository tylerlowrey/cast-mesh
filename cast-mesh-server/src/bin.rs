#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use cast_mesh_server::routes;
use rocket::Config;
use rocket::config::Environment;
use cast_mesh_server::cors::CORS;

const ROCKET_SERVER_PORT: u16 = 50050;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let rocket_server_task = tokio::spawn(async {
        let config = Config::build(Environment::Development)
            .address("127.0.0.1")
            .port(ROCKET_SERVER_PORT)
            .finalize()
            .expect("Unable to configure rocket server");

        let rocket_server = rocket::custom(config).attach(CORS());
        rocket_server.mount("/", routes![
            routes::index,
            routes::register_device,
            routes::get_devices_list,
            routes::get_device_data,
            routes::remove_device,
        ]).launch();
    });

    rocket_server_task.await.expect("Rocket server shutdown unexpectedly.");

    Ok(())
}