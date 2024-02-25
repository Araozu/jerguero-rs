#[macro_use]
extern crate rocket;

mod view;

use maud::{html, Markup};
use rocket::fs::{relative, FileServer};

#[get("/")]
fn index() -> Markup {
    view::skeleton(html! {
        h1 class="font-bold text-4xl text-center text-primary"
        {
            "Sistema EEGSAC"
        }
        br;
        form action="/login" method="post" {
            label class="form-control w-full max-w-xs"
            {
                div class="label"
                {
                    span class="label-text"
                    {
                        "Usuario"
                    }
                }
                input type="text"
                    placeholder="Usuario"
                    required
                    class="input input-bordered input-primary w-full max-w-xs";
            }
            label class="form-control w-full max-w-xs"
            {
                div class="label"
                {
                    span class="label-text"
                    {
                        "Contraseña"
                    }
                }
                input type="password"
                    placeholder="Contraseña"
                    required
                    class="input input-bordered input-primary w-full max-w-xs";
            }
            button class="btn btn-primary"
            {
                "Iniciar sesión"
            }
        }
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
}
