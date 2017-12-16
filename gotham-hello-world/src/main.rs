//! A Hello World example application for working with [Gotham](https://gotham.rs).
//! Useful as a very light touch introduction to Gotham or as a point to build from
//! when you have very simple requirements and don't need a Router to handle
//! dispatch, middleware, pipelines etc.

//! For an example of working with Gotham when you have more complex requirements
//! please see our full
//! [example application](https://github.com/gotham-rs/example-app) which
//! includes usage of the Gotham Router and other advanced Gotham components.

// The code below allows us to make and recieve the following Request/Response:
//
// $ cargo run
// Listening on http://127.0.0.1:7878 with 1 thread.
//
// ...
//
// $ curl -v http://127.0.0.1:7878/
// *   Trying 127.0.0.1...
// * TCP_NODELAY set
// * Connected to 127.0.0.1 (127.0.0.1) port 7878 (#0)
// > GET / HTTP/1.1
// > Host: 127.0.0.1:7878
// > User-Agent: curl/7.54.1
// > Accept: */*
// >
// < HTTP/1.1 200 OK
// < Content-Length: 12
// < Content-Type: text/plain
// < X-Request-ID: a6d7052c-41c0-465a-b5ae-a0d412eb7ee3
// < X-Frame-Options: DENY
// < X-XSS-Protection: 1; mode=block
// < X-Content-Type-Options: nosniff
// < X-Runtime-Microseconds: 186
// < Date: Tue, 15 Aug 2017 23:53:46 GMT
// <
// * Connection #0 to host 127.0.0.1 left intact
// Hello World!%

extern crate futures;
extern crate hyper;
extern crate gotham;
extern crate mime;

use hyper::server::Http;
use hyper::{Request, Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::handler::NewHandlerService;

/// This is an example of a Gotham `Handler`.
///
/// You can read more about `Handlers` in [the Gotham Book](https://book.gotham.rs/usage/handlers/start.html).
///
/// The most interesting part of this is that the actual Gotham Router is just a `Handler` as well.
/// So to start using the Router you'd simply create one and pass it to NewServiceHandler instead of
/// `say_hello` as we've done here.
///
/// This `Handler` will always respond regardless of the `Request` path. Dealing with breaking up
/// the Request path and dispatching to application code is a one part of what the full Gotham
/// `Router` offers. You could do some direct matches on `Request` path here and call different
/// functions if you have very simple requirements.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait
/// [for functions that match the signature used here](https://github.com/gotham-rs/gotham/blob/d11c9b9331627c5ea2119aeb2957166c9248915f/src/handler/mod.rs#L398)
/// within Gotham itself.
pub fn say_hello(state: State, _req: Request) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("Hello World!").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );

    (state, res)
}

/// Create a new server instance and have Hyper call the Gotham `Handler` we've defined above for
/// every new `Request` it recieves.
pub fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    let server = Http::new()
        .keep_alive(true)
        .pipeline(true)
        .bind(&addr, NewHandlerService::new(|| Ok(say_hello)))
        .unwrap();

    println!(
        "Listening on http://{}",
        server.local_addr().unwrap()
    );

    server.run().unwrap();
}
