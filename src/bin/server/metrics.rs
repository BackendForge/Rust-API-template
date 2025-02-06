use std::{
    future::ready,
    net::{IpAddr, SocketAddr},
};

use api_lib::error::Error;
use axum::{routing::get, Router};
use axum_server::Handle;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing::info;

static PROMETHEUS_HANDLE: Lazy<Mutex<Option<PrometheusHandle>>> = Lazy::new(|| Mutex::new(None));

pub async fn bind_metrics(handle: Handle, host: IpAddr, port: u16) -> Result<(), Error> {
    let recorder_handle = setup_metrics_recorder();

    match recorder_handle {
        Some(rh) => {
            let router = Router::new().route("/metrics", get(move || ready(rh.render())));
            let addr = SocketAddr::from((host, port));

            info!("Listening for metrics http on {host}:{port}");

            // Bind the http server address and serve the router
            axum_server::bind(addr)
                .handle(handle)
                .serve(router.into_make_service())
                .await?;

            Ok(())
        }
        None => {
            info!("metrics recorder already exists");
            Ok(())
        }
    }
}

fn setup_metrics_recorder() -> Option<PrometheusHandle> {
    let mut handle_lock = PROMETHEUS_HANDLE.lock().unwrap();
    if handle_lock.is_none() {
        const EXPONENTIAL_SECONDS: &[f64] = &[
            0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ];

        match PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full("http_requests_duration_seconds".to_string()),
                EXPONENTIAL_SECONDS,
            )
            .unwrap()
            .install_recorder()
        {
            Ok(handle) => {
                *handle_lock = Some(handle.clone());
                Some(handle)
            }
            Err(_) => None, // Don't panic if another recorder exists
        }
    } else {
        handle_lock.clone()
    }
}
