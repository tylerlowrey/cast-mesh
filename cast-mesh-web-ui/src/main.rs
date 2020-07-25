use rusqlite::Connection;
use rocket::Config;
use cast_mesh_ui_backend::ROCKET_SERVER_PORT;
use rocket::config::Environment;
use cast_mesh_ui_backend::cors::CORS;

fn main() {

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
            timestamp BIGINT NOT NULL,
            data INTEGER NOT NULL
        )"#,
        params![]
    ).expect("Unable to create table for device data");

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
}
