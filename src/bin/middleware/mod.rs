use std::time::Duration;

use ::tracing::debug_span;
use api_lib::runtime::Args;
use authentication::handle_authentication;
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{header, HeaderMap, Method, Request},
    middleware,
    response::Response,
    Router,
};
use metrics::handle_metrics;
use status::handle_status;
use tower::ServiceBuilder;
use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, Span};

mod authentication;
mod metrics;
mod status;
pub trait Middleware {
    fn with_middleware(self, args: Args) -> Self
    where
        Self: Sized;
}

impl<T> Middleware for Router<T>
where
    T: Clone + Sync + Send + 'static,
{
    fn with_middleware(self, args: Args) -> Self
    where
        Self: Sized,
    {
        // Create a new ServiceBuilder and insert it's middleware layers
        let middleware_layer = ServiceBuilder::new()
            .layer(middleware::from_fn(handle_status))
            .layer(middleware::from_fn(handle_authentication))
            .layer(middleware::from_fn(handle_metrics));

        // Create a new CorsLayer and insert the content policy argument values
        let cors_layer = CorsLayer::new()
            .allow_headers([header::CONTENT_TYPE])
            .allow_methods([Method::GET])
            .allow_origin(args.allow_origin());

        // Create a trace layer and insert all logs
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                debug_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(|request: &Request<_>, _: &Span| {
                debug!("started {} {}", request.method(), request.uri().path())
            })
            .on_response(|_: &Response, latency: Duration, _: &Span| {
                debug!("response generated in {:?}", latency)
            })
            .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
                debug!("sending {} bytes", chunk.len())
            })
            .on_eos(
                |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                    debug!("stream closed after {:?}", stream_duration)
                },
            )
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    debug!("something went wrong")
                },
            );

        // Insert the middleware and cors layer into the current router
        self.route_layer(middleware_layer)
            .layer(cors_layer)
            .layer(trace_layer)
    }
}
