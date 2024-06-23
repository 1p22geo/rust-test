use api::api;
use rocket::get;
use rocket::launch;
use rocket::routes;
use rocket::uri;
use rocket::{fs::FileServer, response::Redirect};

mod api;

#[get("/")]
fn index() -> Redirect {
    Redirect::temporary(uri!("./static/index.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", api())
        .mount("/static", FileServer::from("./static"))
}
