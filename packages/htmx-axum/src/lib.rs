use async_trait::async_trait;
use axum::{
    body::{Bytes, HttpBody},
    extract::FromRequest,
    headers::ContentType,
    http::{self, Request},
    response::{IntoResponse, Response},
    BoxError, TypedHeader,
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

pub struct HtmxPostRequest<T>(pub T);

#[async_trait]
impl<State, Body, T> FromRequest<State, Body> for HtmxPostRequest<T>
where
    State: Send + Sync,
    Body: HttpBody + Send + 'static,
    Body::Data: Send,
    Body::Error: Into<BoxError>,
    T: DeserializeOwned,
{
    type Rejection = http::StatusCode;

    async fn from_request(req: Request<Body>, state: &State) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| http::StatusCode::BAD_REQUEST)?;
        serde_urlencoded::from_bytes(&bytes)
            .map_err(|_| http::StatusCode::BAD_REQUEST)
            .map(HtmxPostRequest)
    }
}
