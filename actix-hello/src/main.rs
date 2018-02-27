extern crate actix;
extern crate actix_web;

use actix_web::*;

fn main() {
    let sys = actix::System::new("test");

    HttpServer::new(
        || Application::default()
            .resource(
                "/", |r| r.f(|_| HttpResponse::Ok()
                             .content_type("text/plain; charset=utf-8").body("Hello World!"))))
        .threads(1)
        .bind("127.0.0.1:8080").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
