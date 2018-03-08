#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::content::Html;

#[get("/", format = "text/html")]
fn html() -> Html<&'static str> {
    let foo = include_str!("web/index.html");
    Html(foo)
}

#[get("/index.js", format = "application/javascript")]
fn javascript() -> &'static str {
    include_str!("web/index.js")
}

fn main() {
    rocket::ignite().mount("/", routes![html, javascript]).launch();
}
