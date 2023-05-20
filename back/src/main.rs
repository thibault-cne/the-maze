use std::collections::HashMap;
use rocket::tokio::sync::Mutex;

use clients::Client;

#[macro_use] extern crate rocket;

mod api;
mod clients;
mod maze;

struct App {
    id_generator: snowflake::SnowflakeIdGenerator,

    clients: HashMap<String, Client>,
}

#[launch]
fn server() -> _ {
    rocket::build()
        .manage(Mutex::new(App::new()))
        .mount("/", api::routes())
        .mount("/client", api::client::routes())
}

impl App {
    pub fn new() -> Self {
        Self {
            id_generator: snowflake::SnowflakeIdGenerator::new(1, 1),
            clients: HashMap::new(),
        }
    }

    pub fn new_client(&mut self, name: String) -> String {
        let id = self.id_generator.real_time_generate().to_string();

        self.clients.insert(id.clone(), Client::new(id.clone(), name));

        id
    }
}
