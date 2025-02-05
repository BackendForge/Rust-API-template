use crate::error::Error;
use clap::Parser;
mod args;

pub use args::{Args, TlsArgs};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Runtime {
    args: Args,
    // postgres: PostgresClient,
}

#[allow(dead_code)]
impl Runtime {
    pub fn parse() -> Result<Self, Error> {
        Ok(Self {
            args: Args::parse(),
            // postgres: PostgresClient::from_env()?,
        })
    }

    pub fn args(&self) -> Args {
        self.args.clone()
    }

    // pub fn postgres(&self) -> PostgresClient {
    //     self.postgres.clone()
    // }
}
