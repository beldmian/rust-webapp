#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(FromForm)]
struct Date {
    date: String,
    save: String,
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert("title".to_string(), "Hello".to_string());
    Template::render("index", &context)
}

#[get("/date/<date>/<save>")]
fn date_show(date: String, save: String) -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert("title".to_string(), date);
    context.insert("save".to_string(), save);
    Template::render("date", context)
}
#[post("/", data = "<date>")]
fn date(date: Form<Date>) -> Redirect {
    println!("{} {}", date.date, date.save);
    Redirect::to(uri!(date_show: &date.date, &date.save))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, date, date_show])
        .attach(Template::fairing())
        .launch();
}
