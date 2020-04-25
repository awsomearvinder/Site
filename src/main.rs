#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket::{get, routes};

use rocket_contrib::{serve::StaticFiles, templates::Template};

mod site;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", StaticFiles::from("static"))
        .mount("/", routes![index, blog, home])
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context = site::SitePage::new(site::Home::new());
    Template::render("index", &context)
}

#[get("/Home")]
fn home() -> Template {
    index()
}
#[get("/Blog")]
fn blog() -> Template {
    let context = site::SitePage::new(site::BlogHome::new());
    Template::render("blog_home", &context)
}
