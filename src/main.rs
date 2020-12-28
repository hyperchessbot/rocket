#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
		.mount("/", routes![index])
		.mount("/", StaticFiles::from("."))
		.launch();
}
