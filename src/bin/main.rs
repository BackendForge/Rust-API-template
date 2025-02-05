mod middleware;
mod routes;
mod server;
use api_lib::{error::Error, runtime::Runtime};
use axum::{routing::post, Router};
use middleware::Middleware;
use routes::*;
use tracing::{error, info};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, registry};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse the runtime arguments and initialize its clients
    let runtime = Runtime::parse()?;

    registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a new router with routes, middleware and state
    let app_router = Router::new()
        .route("/auth/signup", post(auth::signup))
        .route("/auth/login", post(auth::login))
        .with_middleware(runtime.args())
        .with_state(runtime.clone())
        .fallback(handler_404);

    // Bind and start the server
    match server::bind(runtime.args(), app_router).await {
        Ok(()) => info!("Server stopped gracefully"),
        Err(err) => error!("{err}"),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, Router};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_404() {
        let app_router = Router::new().fallback(handler_404);

        let response = app_router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), 404);
    }
}
