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
                // Google fonts: Playfair Display
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
            div class="w-full h-16" {}
        }
    })
}

fn post() -> Markup {
    html! {
        article class="card card-compact shadow-md my-2 border border-[rgba(150,150,150,0.25)]" {
            div class="card-body" {
                div class="grid grid-cols-[auto_4rem]" {
                    a href="#" class="text-blue-600 font-serif font-black hover:underline" {
                        h2 class="card-title" {
                            "Pololo/a "
                        }
                    }
                    div class="inline-block text-right text-xl" { 
                        span title="Controversial" {"🔥"}
                        " "
                        span title="Utilizado en Chile" {"🇨🇱"}
                    }
                }
                div {
                    div class="badge badge-ghost text-xs hover:bg-neutral hover:text-neutral-content cursor-pointer" {"Amor"}
                    div class="badge badge-ghost text-xs hover:bg-neutral hover:text-neutral-content cursor-pointer" {"Relaciones"}
                }
                p {
                    "Otra palabra para decir enamorado/a"
                }
                br;
                div class="grid grid-cols-[auto_3.25rem]" {
                    div class="opacity-70 text-xs" {
                        a href="#" class="inline-block underline font-serif font-semibold" { "Pablito" }
                        " · "
                        span { "05/12/2022" }
                    }
                    div class="text-lg text-right" {
                        i class="ph ph-thumbs-down hover:underline cursor-pointer" {}
                        " "
                        i class="ph ph-thumbs-up hover:underline cursor-pointer" {}
                    }
                }
            }
        }
    }
}
