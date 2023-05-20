use rocket::Route;
use serde_json::{Value, Map};

pub mod client;
mod response;

pub static COOKIE_NAME: &'static str = "the_maze";

pub trait Object {
    fn to_map(&self) -> Map<String, Value>;
}

#[get("/status")]
fn status() -> &'static str {
    "Up and running!"
}

pub fn routes() -> Vec<Route> {
    routes![status]
}
