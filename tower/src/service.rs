use std::fmt;
use std::task::{Context, Poll};
use tower_service::Service;


pub struct HelloService<S> {
    pub(crate) target: &'static str,
    pub(crate) service: S,
}

pub struct Hello2Service<S> {
    pub(crate) target: &'static str,
    pub(crate) service: S,
}

impl<S, Request> Service<Request> for HelloService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("HelloService: request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}

impl<S, Request> Service<Request> for Hello2Service<S>
    where
        S: Service<Request>,
        Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("Hello2Service: request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}