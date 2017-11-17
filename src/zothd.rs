// TODO: not work, now
extern crate env_logger;
extern crate actix;
extern crate actix_web;
extern crate futures;

use actix::{msgs, Arbiter};
use actix_web::*;

fn index(req: &mut HttpRequest, mut _payload: Payload, _state: &()) -> HandlerResult<HttpResponse> {
    let cmdline = req.match_info().get("cmdline").unwrap();
    println!("{:?}, {:?}", req, cmdline);
    Ok(HttpResponse::builder(StatusCode::OK)
       .content_type("text/html")
       .body(format!("Hello {}!", req.match_info().get("name").unwrap()))
       .unwrap()
       .into()).into()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();
    let sys = actix::System::new("zothd");

    HttpServer::new(
        Application::default("/")
            .middleware(middlewares::Logger::default())
            .resource("/", |r| r.handler(Method::POST, index))
        ).serve::<_, ()>("127.0.0.1:8080").unwrap();

    println!("Started http server: 127.0.0.1:8080");
    Arbiter::system().send(msgs::SystemExit(0));
    let _ = sys.run();
}
