extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    let mut i = Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world!")))
    });
    i.threads = 8;

    i.http("localhost:8080").unwrap();
}
