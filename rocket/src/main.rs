#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::config::{Config, Environment};

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    let config = Config::build(Environment::Production)
        .address("127.0.0.1")
        .port(8080)
        .workers(8)
        .unwrap();
    rocket::custom(config, false).mount("/", routes![hello]).launch();
}
