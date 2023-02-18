
use tower::{BoxError, service_fn, ServiceBuilder, ServiceExt};
use tower_service::Service;

mod service;
mod layer;

#[derive(Debug)]
struct Request;

impl Request {
    fn new() -> Self { Self }
}

struct Response(&'static str);

impl Response {
    fn new(body: &'static str) -> Self {
        Self(body)
    }
    fn into_body(self) -> &'static str { self.0 }
}

#[tokio::test]
async fn test_service_layer() -> Result<(), BoxError> {
    async fn handle(_request: Request) -> Result<Response, BoxError> {
        let response = Response::new("Hello, World!");
        Ok(response)
    }
    let mut svc = ServiceBuilder::new()
        .layer(layer::OneLayer::new())
        .layer(layer::TwoLayer::new())
        .service(service_fn(handle));
    let response = svc.ready()
        .await?
        .call(Request::new())
        .await?;
    println!("{}", response.into_body());
    Ok(())
}
