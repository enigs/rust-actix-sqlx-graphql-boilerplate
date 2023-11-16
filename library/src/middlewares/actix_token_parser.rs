use actix_utils::future::{ok, Either, Ready};
use actix_web::{Error, Result,  http::Method};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::{ready, Future};
use pin_project::pin_project;
use std::{marker::PhantomData, pin::Pin, task::{Context, Poll}};

#[derive(Default, Clone)]
pub struct ActixTokenParser {
    is_controller: bool
}

impl ActixTokenParser {
    pub fn controller() -> Self {
        Self {
            is_controller: true
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ActixTokenParser
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ActixTokenParserMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ActixTokenParserMiddleware {
            service,
            is_controller: self.is_controller
        })
    }
}

pub struct ActixTokenParserMiddleware<S> {
    service: S,
    is_controller: bool
}

impl<S, B> Service<ServiceRequest> for ActixTokenParserMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;

    type Error = Error;

    type Future = Either<ActixTokenParserFuture<S, B>, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass = false;

        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        }

        let token = crate::Cipher::from(match req.headers().get("Authorization") {
            Some(data) => data.to_str().unwrap_or_default().to_string(),
            _ => String::default()
        }.replace("Bearer ", ""))
            .decrypt()
            .unwrap_or_default()
            .b64encode()
            .unwrap_or_default();

        let key = std::env::var("MASTER_KEY")
            .unwrap_or_default();

        if !token.is_empty() && !key.is_empty() && self.is_controller && token == key {
            authenticate_pass = true;
        }

        if authenticate_pass {
            Either::left(ActixTokenParserFuture {
                fut: self.service.call(req),
                _phantom: PhantomData,
            })
        } else {
            Either::right(ok(req
                .into_response(config::page::not_found())
                .map_into_boxed_body()
                .map_into_right_body()))
        }
    }
}

#[pin_project]
pub struct ActixTokenParserFuture<S, B>
    where
        S: Service<ServiceRequest>,
{
    #[pin]
    fut: S::Future,
    _phantom: PhantomData<B>,
}

impl<S, B> Future for ActixTokenParserFuture<S, B>
    where
        B: MessageBody,
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<EitherBody<B>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = match ready!(self.project().fut.poll(cx)) {
            Ok(res) => res,
            Err(err) => return Poll::Ready(Err(err)),
        };

        Poll::Ready(Ok(res.map_into_left_body()))
    }
}