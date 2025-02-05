use api_lib::runtime::Args;
use axum::{
    handler::HandlerWithoutStateExt,
    http::{uri::Scheme, Uri},
    response::Redirect,
    BoxError, Router,
};
use axum_extra::extract::Host;
use axum_server::{tls_rustls::RustlsConfig, Handle};
use std::{io, net::SocketAddr};
use tracing::info;

/// Bind both the http and https endpoints and create an automated redirect from http to https if possible
pub async fn bind(
    args: Args,
    tls_port: u16,
    config: RustlsConfig,
    handle: Handle,
    router: Router,
) -> Result<(), io::Error> {
    // Create the https listening addres
    let addr = SocketAddr::from((args.host(), tls_port));

    // Create the http -> https redirect route
    tokio::spawn(redirect_http(handle.clone(), args, tls_port));

    info!("listening for https on port: {}", addr.port());

    // Bind the TLS port to the https endpoint and start the server
    axum_server::bind_rustls(addr, config)
        .handle(handle)
        .serve(router.into_make_service())
        .await
}

/// Turn an http route into an https route
fn make_https(host: String, uri: Uri, http: u16, https: u16) -> Result<Uri, BoxError> {
    let mut src = uri.into_parts();

    src.scheme = Some(Scheme::HTTPS);

    // If path and query are both none, append a trailing slash
    if src.path_and_query.is_none() {
        src.path_and_query = Some("/".parse().unwrap())
    }

    // replace the authority from http to https
    src.authority = Some(
        host.replace(&http.to_string(), &https.to_string())
            .parse()?,
    );

    Ok(Uri::from_parts(src)?)
}

/// The logic function fo the http -> https redirect route
async fn redirect_http(handle: Handle, args: Args, tls_port: u16) -> Result<(), io::Error> {
    // Create the https listening route
    let addr = SocketAddr::from((args.host(), tls_port));

    // Bind the redirect method as a route element
    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, args.port(), tls_port) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(err) => {
                tracing::warn!(%err, "failed to convert URI to HTTPS");

                Err(axum::http::StatusCode::BAD_REQUEST)
            }
        }
    };

    info!("listening for http on port: {}", addr.port());

    // Start the http server that will automatically redirect all trafic to the https endpoint
    axum_server::bind(addr)
        .handle(handle)
        .serve(redirect.into_make_service())
        .await
}
