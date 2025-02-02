use maud::{html, Markup, DOCTYPE};

mod icons;

use icons::{bird, magnifying_glass};

pub fn skeleton(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                title { "Jerguero" }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" type="text/css" href="/static/out.css";
                // Phosphor icons
                link rel="stylesheet"
                    type="text/css"
                    href="https://unpkg.com/@phosphor-icons/web@2.0.3/src/regular/style.css";
                // Radio Canada Big
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Radio+Canada+Big:ital,wght@0,400..700;1,400..700&display=swap" rel="stylesheet";
                // hyperscript
                script src="https://unpkg.com/hyperscript.org@0.9.12" defer {}
            }
            body {
                (body)
            }
        }
    }
}

fn navbar() -> Markup {
    html! {
        nav class="fixed z-50 bottom-0 left-0 w-full h-14" {
            div id="search-results" class="hidden absolute bottom-14 w-full bg-c-bg text-c-on-bg text-white border-t-4 border-l-4 border-r-4 border-c-primary rounded-tr-md rounded-tl-md" {
                (inline_definition())
                (inline_definition())
                (inline_definition())
            }
            div class="grid grid-cols-[3.5rem_auto_3.5rem] bg-c-primary" {
                div class="table" {
                    a href="/" class="table-cell align-middle h-14 text-center" {
                        (bird("white".into(), 36))
                    }
                }
                div class="relative" {
                    svg class="absolute z-10 -left-[15px] top-[9px] scale-x-50" fill="white" width="20" height="20" viewBox="0 0 50 50" {
                        path d="M 57.29217,4.9844012 V 56.568575 C 44.512514,39.388798 29.429048,35.264265 12.536889,30.729102 29.111034,27.220159 45.273777,21.221938 57.29217,4.9844012 Z" {}
                    }
                    input
                        _="on keyup debounced at 300ms
                            set :x to my.value.length
                            if :x > 0
                                remove .hidden from #search-results
                            else
                                add .hidden to #search-results
                            end
                            "
                        type="text"
                        placeholder="Buscá una palabra"
                        class="w-full py-2 px-1 rounded-md my-2 bg-white text-black";
                }
                div class="table" {
                    button class="table-cell align-middle h-14 text-center w-full" {
                        (magnifying_glass("white".into(), 36))
                    }
                }
            }
        }
    }
}

fn inline_definition() -> Markup {
    html! {
        div class="py-2 mx-2" {
            span class="font-bold pr-2" { "Polola" }
            span class="opacity-75" { "Otra palabra para decir enamorado/a" }
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
        article class="bg-c-bg-2 shadow-md my-2 rounded-md py-2 px-4" {
            div class="card-body" {
                div class="grid grid-cols-[auto_4rem]" {
                    a href="#" class="text-c-primary text-2xl font-black hover:underline pt-2 pb-2" {
                        h2 class="card-title" {
                            "Pedro pedro pedro"
                        }
                    }
                    div class="flex justify-end items-center text-right text-xl" {
                        span title="Utilizado en Argentina" {"🇦🇷"}
                    }
                }
                div class="pb-2" {
                    (badge("Amor".into()))
                    (badge("Relaciones".into()))
                }
                p {
                    "Pedro pe"
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

pub fn badge(text: String) -> Markup {
    html! {
        button class="inline-block mr-2 py-1 px-2 text-xs rounded-full hover:underline bg-[#082f49] cursor-pointer" {
            (text)
        }
    }
}
