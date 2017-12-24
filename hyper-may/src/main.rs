#![deny(warnings)]
extern crate may;
extern crate hyper;

use hyper::server::{Request, Response};

static PHRASE: &'static [u8] = b"Hello World!";

fn hello(_: Request, res: Response) {
    res.send(PHRASE).unwrap();
}

fn main() {
    may::config().set_io_workers(2).set_stack_size(0x2000);
    let _listening = hyper::Server::http("127.0.0.1:8080").unwrap()
        .handle(hello);
    println!("Listening on http://127.0.0.1:8080");
}
