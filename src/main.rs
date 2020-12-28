#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::<String, String>::new();
	context.insert("welcome".to_string(), "Welcome to Rocket !".to_string());
    Template::render("index", &context)
}

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
		.mount("/", routes![index])
		.mount("/", StaticFiles::from("./public"))
		.attach(Template::fairing())
		.launch();
}
