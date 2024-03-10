use maud::{html, Markup, DOCTYPE};

pub fn skeleton(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" data-theme="cupcake" {
            head {
                title { "Jerguero" }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" type="text/css" href="/static/out.css";
                // Phosphor icons
                link rel="stylesheet"
                    type="text/css"
                    href="https://unpkg.com/@phosphor-icons/web@2.0.3/src/regular/style.css";
                // Google fonts: Playfair Display and Inter
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Playfair+Display:wght@900&display=swap" rel="stylesheet";
            }
            body {
                (body)
            }
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav class="navbar gap-1 fixed z-50 bottom-0 left-0 w-full border-t border-[rgba(150,150,150,0.5)] bg-base-200 text-base-content" {
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

        div class="container" {
            div class="bg-accent text-accent-content py-2 m-1 rounded-md text-center font-bold text-lg" {
                "Jerguero"
            }
            (post())
            (post())
            (post())
            (post())
        }
    })
}

fn post() -> Markup {
    html! {
        article class="card card-compact shadow-md my-2 border border-[rgba(150,150,150,0.25)]" {
            div class="card-body" {
                a href="#" class="text-blue-600 font-serif font-black" {
                    h2 class="card-title" {
                        "Take the L"
                    }
                }
                p {
                    "Stands for \"Take the loss\". Frequently used to describe flunking a test, being dumped, being stood up, being beaten up or robbed, or losing one's money in the stock market, gambling, or through exploitative business schemes."
                }
                br;
                div class="opacity-80 text-sm" {
                    a href="#" class="inline-block underline font-serif font-semibold" { "Pablito" }
                    " · "
                    span { "05/12/2022" }
                }
            }
        }
    }
}
