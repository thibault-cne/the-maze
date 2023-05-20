use rocket::http::{Status, ContentType};
use rocket::Request;
use rocket::response::{self, Responder};
use std::io;
use serde_json::{Value, json, Map};

use crate::api::Object;

pub struct Response {
    status: Status,

    content: Option<Map<String, Value>>,
}

impl Response {
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn add_field(&mut self, key: &str, value: &str) {
        if self.content.is_none() {
            self.content = Some(Map::new());
        }

        self.content.as_mut().unwrap().insert(key.to_string(), json!(value));
    }

    pub fn add_object<O>(&mut self, key: &str, object: &O)
        where O: Object {
        if self.content.is_none() {
            self.content = Some(Map::new());
        }

        let map = object.to_map();
        self.content.as_mut().unwrap().insert(key.to_string(), Value::Object(map));
    }


}

impl From<Status> for Response {
    fn from(status: Status) -> Self {
        Self {
            status,
            content: None,
        }
    }
}

impl From<u16> for Response {
    fn from(status: u16) -> Self {
        Self {
            status: Status::from_code(status).unwrap(),
            content: None,
        }
    }
}

impl<'r> Responder<'r, 'static> for Response {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let mut response = response::Response::new();

        response.adjoin_header(ContentType::JSON);
        response.set_status(self.status);

        if self.content.is_some() {
            let content = self.content.unwrap();
            let content = serde_json::to_string(&content).unwrap();
            response.set_sized_body(content.len(), io::Cursor::new(content));
        }


        Ok(response)
    }
}
