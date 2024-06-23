use rocket::get;
use rocket::post;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::Route;
use std::process::Command;

static SYSTEM: &'static str = "Arch Linux";

pub fn get_release() -> String {
    let out = Command::new("/bin/sh")
        .args(["-c", "uname -r"])
        .output()
        .expect("Failed to get release");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[derive(Serialize, Deserialize)]
struct ApiDataResponse {
    system: &'static str,
    release: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ApiDataRequest {
    system: String,
    release: String,
}

#[get("/data")]
fn api_data() -> Json<ApiDataResponse> {
    Json(ApiDataResponse {
        system: SYSTEM,
        release: get_release(),
    })
}

#[post("/data", data = "<data>")]
fn api_echo(data: Json<ApiDataRequest>) -> Json<ApiDataRequest> {
    Json(ApiDataRequest {
        system: data.system.to_owned(),
        release: data.release.to_owned(),
    })
}

pub fn api() -> Vec<Route> {
    routes![api_data, api_echo]
}
