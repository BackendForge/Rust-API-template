use api_lib::runtime::Args;
use axum::Router;
use axum_server::Handle;
use std::{io, net::SocketAddr};
use tracing::info;

/// Bind the http server endpoint and start the server
pub async fn bind(args: Args, handle: Handle, router: Router) -> Result<(), io::Error> {
    let addr = SocketAddr::from((args.host(), args.port()));

    info!("listening for http on port: {}", addr.port());

    // Bind the http server address and serve the router
    axum_server::bind(addr)
        .handle(handle)
        .serve(router.into_make_service())
        .await
}
