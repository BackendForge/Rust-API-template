use axum::http::HeaderValue;
use clap::Parser;
use std::{net::IpAddr, path::PathBuf};

#[derive(Parser, Clone)]
#[clap(group(
    clap::ArgGroup::new("metrics")
        .required(false)
        .multiple(true)
        .requires_all(&["metrics_port", "metrics_host"])
        .args(&["metrics_port", "metrics_host"])
))]
pub struct MetricsArgs {
    #[arg(long("metrics-port"), env("METRICS_PORT"))]
    metrics_port: Option<u16>,

    #[arg(long("metrics-host"), env("METRICS_HOST"))]
    metrics_host: Option<IpAddr>,
}

impl MetricsArgs {
    pub fn port(&self) -> u16 {
        self.metrics_port.unwrap()
    }

    pub fn host(&self) -> IpAddr {
        self.metrics_host.unwrap()
    }

    pub fn opt(&self) -> Option<(IpAddr, u16)> {
        self.metrics_host.zip(self.metrics_port)
    }
}

#[derive(Parser, Clone)]
#[clap(group(
    clap::ArgGroup::new("tls")
        .required(false)
        .multiple(true)
        .requires_all(&["https_port", "cert_path", "key_path"])
        .args(&["https_port", "cert_path", "key_path"])
))]
pub struct TlsArgs {
    #[arg(long("tls-port"), env("HTTPS_PORT"))]
    https_port: Option<u16>,

    #[arg(long("tls-cert"), env("TLS_CERT_PATH"))]
    cert_path: Option<PathBuf>,

    #[arg(long("tls-key"), env("TLS_KEY_PATH"))]
    key_path: Option<PathBuf>,
}

impl TlsArgs {
    pub fn port(&self) -> u16 {
        self.https_port.unwrap()
    }

    pub fn cert_path(&self) -> PathBuf {
        self.cert_path.clone().unwrap()
    }

    pub fn key_path(&self) -> PathBuf {
        self.key_path.clone().unwrap()
    }

    pub fn opt(&self) -> Option<(u16, PathBuf, PathBuf)> {
        self.https_port
            .zip(self.cert_path.clone())
            .zip(self.key_path.clone())
            .map(|((port, cert), key)| (port, cert, key))
    }
}

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[arg(long, env("HOST"), default_value_t = [0,0,0,0].into())]
    host: IpAddr,

    #[arg(long, env("HTTP_PORT"), default_value_t = 7070)]
    http_port: u16,

    #[arg(long, env("ALLOW_ORIGIN"), value_parser = HeaderValue::from_str, default_value = "*")]
    allow_origin: HeaderValue,

    #[clap(flatten)]
    metrics: MetricsArgs,

    #[clap(flatten)]
    tls: TlsArgs,
}

impl Args {
    pub fn host(&self) -> IpAddr {
        self.host
    }

    pub fn port(&self) -> u16 {
        self.http_port
    }

    pub fn allow_origin(&self) -> HeaderValue {
        self.allow_origin.clone()
    }

    pub fn metrics(&self) -> MetricsArgs {
        self.metrics.clone()
    }

    pub fn tls(&self) -> TlsArgs {
        self.tls.clone()
    }
}
