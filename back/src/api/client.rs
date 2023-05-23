use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::http::{CookieJar, Cookie, Status};
use rocket::form::Form;

use crate::App;
use crate::maze::cells::Direction;
use crate::api::response::Response;
use crate::api::COOKIE_NAME;

#[get("/")]
async fn status(app: &State<Mutex<App>>, jar: &CookieJar<'_>) -> Response {
    if let Some(cookie) = jar.get(COOKIE_NAME) {
        let locked_app = app.lock().await;
        let client = locked_app.clients.get(cookie.value()).expect("Couldn't get client");

        let mut resp = Response::from(Status::Ok);
        resp.add_object("client", client);
        return resp;
    }

    return Response::from(Status::Unauthorized);
}

#[derive(FromForm)]
struct LoginForm<'r> {
    username: &'r str,
}

#[post("/login", data = "<login>")]
async fn login(app: &State<Mutex<App>>, jar: &CookieJar<'_>, login: Form<LoginForm<'_>>) -> Response {
    let mut locked_app = app.lock().await;
    let id = locked_app.new_client(login.username.to_string());

    jar.add(Cookie::new(COOKIE_NAME, id));

    Response::from(Status::Ok)
}

#[post("/new_game")]
async fn new_game(app: &State<Mutex<App>>, jar: &CookieJar<'_>) -> Response {
    if let Some(cookie) = jar.get(COOKIE_NAME) {
        let mut locked_app = app.lock().await;
        let client = locked_app.clients.get_mut(cookie.value()).expect("Couldn't get client");
        let _ = client.start_game();

        let mut resp = Response::from(Status::Ok);
        resp.add_object("client", client);
        return resp;
    }

    Response::from(Status::Unauthorized)
}

#[derive(FromForm)]
struct MoveForm<'r> {
    direction: &'r str,
}

#[post("/move", data = "<direction>")]
async fn move_client(app: &State<Mutex<App>>, jar: &CookieJar<'_>, direction: Form<MoveForm<'_>>) -> Response {
    let mut resp = Response::from(Status::Unauthorized);

    if let Some(cookie) = jar.get(COOKIE_NAME) {
        let mut locked_app = app.lock().await;
        let client = locked_app.clients.get_mut(cookie.value()).expect("Couldn't get client");
        
        if !client.is_playing {
            resp.set_status(Status::BadRequest);
            resp.add_field("status", "client not playing");
            return resp;
        }

        let dir = Direction::from_str(direction.direction);

        if dir.is_none() {
            resp.set_status(Status::BadRequest);
            resp.add_field("status", "incorrect direction");
            return resp;
        }

        let movement = client.move_cell(dir.unwrap());

        // TODO: return a json of the deplacement
        resp.set_status(Status::Ok);
        resp.add_field("movement", &movement.to_string());
        resp.add_field("direction", &direction.direction);
        resp.add_object("client", client);
        return resp;
    }
    
    resp.add_field("status", "You are not logged in");
    resp
}

pub fn routes() -> Vec<rocket::Route> {
    routes![status, login, new_game, move_client]
}