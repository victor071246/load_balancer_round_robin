use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

pub async fn get_servers_up(port: u16, name: &'static str) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(move |_conn | async move {
        Ok::<_, Infallible>(service_fn(move |_req: Request<Body>| async move {
            Ok::<_, Infallible>(
                Response::new(Body::from(format!("olá ")))
            )
        }))
    });

    Server::bind(&addr).serve(make_svc).await.unwrap();
}