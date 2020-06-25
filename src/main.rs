extern crate server;
use server::config::*;
use server::server::run;
use tokio::prelude::*;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut rt = Runtime::new().unwrap();

    let config = load_config();
    run(config).await;
}
