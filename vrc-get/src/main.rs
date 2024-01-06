#![deny(clippy::wildcard_enum_match_arm)]

extern crate core;

use clap::Parser;
use reqwest::Client;

mod commands;

#[tokio::main]
async fn main() {
    init_log();
    commands::Command::parse().run().await;
}

fn init_log() {
    use env_logger::*;
    init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
}

pub(crate) fn create_client(offline: bool) -> Option<Client> {
    if offline {
        None
    } else {
        Some(
            Client::builder()
                .user_agent(concat!("vrc-get/", env!("CARGO_PKG_VERSION")))
                .build()
                .expect("building client"),
        )
    }
}
