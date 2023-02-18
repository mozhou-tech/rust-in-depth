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
        println!("HelloService-1: request = {:?}, target = {:?}", request, self.target);
        let fu = self.service.call(request);
        println!("HelloService-2");
        fu
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
        // 洋葱模型
        println!("Hello2Service-1: request = {:?}, target = {:?}", request, self.target);
        let fu = self.service.call(request);
        println!("Hello2Service-2");
        fu
    }
}