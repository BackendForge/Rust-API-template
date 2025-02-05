mod http;
mod https;
mod metrics;

use api_lib::{error::Error, runtime::Args};
use axum::Router;
use axum_server::{tls_rustls::RustlsConfig, Handle};
use metrics::bind_metrics;
use rustls::crypto::ring::default_provider;
use std::time::Duration;
use tokio::signal;

/// Bind the server to the listening host and port and start the server
pub async fn bind(args: Args, router: Router) -> Result<(), Error> {
    let handle = Handle::new();

    if let Some((metrics_host, metrics_port)) = args.metrics().opt() {
        let metrics = bind_metrics(handle.clone(), metrics_host, metrics_port);

        tokio::spawn(metrics);
    }

    tokio::spawn(bind_shutdown(handle.clone(), Some(Duration::from_secs(10))));

    // If TLS args are provided, open a https listener with a http redirected to it,
    // else open a regular http listener
    match args.tls().opt() {
        Some((tls_port, tls_cert, tls_key)) => {
            default_provider()
                .install_default()
                .expect("Failed to install rustls crypto provider");

            let config = RustlsConfig::from_pem_file(tls_cert, tls_key).await?;

            https::bind(args, tls_port, config, handle, router).await
        }
        None => http::bind(args, handle, router).await,
    }?;

    Ok(())
}

/// Shutdown handle
async fn bind_shutdown(handle: Handle, _duration: Option<Duration>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => handle.graceful_shutdown(_duration),
        _ = terminate => handle.shutdown(),
    }
}
