#[macro_use]
extern crate rocket;

mod controller;
mod view;

use rocket::fs::{relative, FileServer};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![controller::homepage])
        .mount("/static", FileServer::from(relative!("static")))
}
