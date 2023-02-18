use std::marker::PhantomData;

use tower_layer::Layer;
use tower_service::Service;

use crate::service::{Hello2Service, HelloService};

pub(crate) struct OneLayer<Request> {
    target: &'static str,
    _p: PhantomData<fn(Request)>,
}

pub(crate) struct TwoLayer<Request> {
    target: &'static str,
    _p: PhantomData<fn(Request)>,
}

impl<Request> OneLayer<Request> {
    pub fn new() -> Self {
        Self {
            target: "OneLayer",
            _p: PhantomData,
        }
    }
}

impl<Request> TwoLayer<Request> {
    pub fn new() -> Self {
        Self {
            target: "TwoLayer",
            _p: PhantomData,
        }
    }
}

impl<S, Request> Layer<S> for OneLayer<Request>
    where
        S: Service<Request> + Send + 'static,
        S::Future: Send,
        S::Error: Into<crate::BoxError> + Send + Sync,
        Request: Send + 'static,
{
    type Service = HelloService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        // 转为另一个Service
       HelloService {
            target: self.target,
            service: inner,
        }
    }
}

impl<S, Request> Layer<S> for TwoLayer<Request>
    where
        S: Service<Request> + Send + 'static,
        S::Future: Send,
        S::Error: Into<crate::BoxError> + Send + Sync,
        Request: Send + 'static,
{
    // 目标Service
    type Service = Hello2Service<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Hello2Service {
            target: self.target,
            service: inner,
        }
    }
}
