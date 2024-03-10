use maud::{html, Markup, DOCTYPE};

pub fn skeleton(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" data-theme="cupcake" {
            head {
                title { "Jerguero" }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" type="text/css" href="/static/out.css";
            }
            body {
                (body)
            }
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav class="navbar gap-1 fixed bottom-0 left-0 w-full border-t border-t-neutral" {
            div class="flex-none" {
                a class="btn btn-ghost text-xl" {
                    "J"
                }
            }
            div class="flex-1" {   
                div class="form-control" {
                    input type="text" placeholder="Search" class="input input-bordered w-full md:w-auto";
                }
            }
            div class="flex-none" {
                a class="btn btn-ghost text-xl" {
                    "s"
                }
            }
        }
    }
}

pub fn homepage() -> Markup {
    skeleton(html! {
        (navbar())
        h1 { "Jerguero" }
        p { "¡Bienvenido a Jerguero!" }
    })
}
