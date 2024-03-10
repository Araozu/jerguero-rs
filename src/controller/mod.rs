use maud::Markup;

#[get("/")]
pub fn homepage() -> Markup {
    crate::view::homepage()
}
