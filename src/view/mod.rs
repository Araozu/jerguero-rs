use maud::{html, Markup, DOCTYPE};

pub fn skeleton(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" data-theme="dracula" {
            head {
                title { "Sistema :D" }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" type="text/css" href="/static/styles.css";
            }
            body {
                (body)
            }
        }
    }
}
