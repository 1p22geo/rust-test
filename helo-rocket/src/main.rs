use rocket::{fs::FileServer, response::Redirect};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Redirect {
    Redirect::temporary(uri!("./static/index.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("./static"))
}
