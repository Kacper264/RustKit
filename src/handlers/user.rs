use hyper::{Body, Request, Response, Error};
use std::sync::Arc;
use prometheus::IntCounter;

pub async fn handle_request(req: Request<Body>, counter: Arc<IntCounter>) -> Result<Response<Body>, Error> {
    counter.inc();

    match req.uri().path() {
        "/hello" => Ok(Response::new(Body::from("Hello, World!\n"))),
        "/bonjour" => Ok(Response::new(Body::from("Bonjour!\n"))),
        _ => Ok(Response::new(Body::from("Bad endpoint"))),
    }
}
