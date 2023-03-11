use std::io;

use async_trait::async_trait;
use axum::{
    extract::FromRequest,
    headers::ContentType,
    http::{self, Request},
    response::{IntoResponse, Response},
    TypedHeader,
};
use serde::de::DeserializeOwned;
use silkenweb::{dom::Dry, prelude::Node};

pub struct HtmxResponse(Node<Dry>);

impl HtmxResponse {
    pub fn new(node: impl Into<Node<Dry>>) -> Self {
        Self(node.into())
    }
}

impl IntoResponse for HtmxResponse {
    fn into_response(self) -> Response {
        (TypedHeader(ContentType::html()), self.0.to_string()).into_response()
    }
}

pub struct HtmxPostRequest<T>(T);

#[async_trait]
impl<State, Body, T> FromRequest<State, Body> for HtmxPostRequest<T>
where
    State: Send + Sync,
    Body: Send + 'static,
    for<'a> &'a Body: io::Read,
    T: DeserializeOwned,
{
    type Rejection = http::StatusCode;

    async fn from_request(req: Request<Body>, _state: &State) -> Result<Self, Self::Rejection> {
        serde_urlencoded::from_reader(req.body())
            .map_err(|_| http::StatusCode::BAD_REQUEST)
            .map(HtmxPostRequest)
    }
}
