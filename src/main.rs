#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket_contrib::{serve::StaticFiles, templates::Template};

mod site;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", StaticFiles::from("static"))
        .mount("/Blog", routes![blog])
        .mount("/", routes![index])
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context = site::SitePage::new(site::Home::new());
    Template::render("index", &context)
}
#[get("/Blog")]
fn blog() -> String {
    String::from("<h1> Fuck you stevie. </h1>")
}
