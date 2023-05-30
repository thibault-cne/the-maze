use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS {
    allowed_origins: Vec<String>,
}

impl CORS {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self {
            allowed_origins,
        }
    }
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        request.headers().get_one("Origin").map(|origin| {
            if self.allowed_origins.contains(&origin.to_string()) {
                response.set_header(Header::new("Access-Control-Allow-Origin", origin));
            }
        });

        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

pub fn routes() -> Vec<rocket::Route> {
    routes![all_options]
}
