#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/template")]
fn template() -> Template {
    let mut context = HashMap::<String, String>::new();
	context.insert("foo".to_string(), "bar".to_string());
    Template::render("index", &context)
}

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
		.mount("/", routes![index, template])
		.mount("/", StaticFiles::from("."))
		.attach(Template::fairing())
		.launch();
}
