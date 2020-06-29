extern crate server;
use server::config::*;
use server::server::run;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    create_dir_if_dont_exist!("./tmp", "./database").await;
    pretty_env_logger::init();

    let config = load_config();
    run(config).await;
}

#[macro_export]
macro_rules! create_dir_if_dont_exist {
    ( $( $x:expr ),* ) => {
        async move {
            use tokio::fs::create_dir;
            $(
                match create_dir($x).await {
                    Ok(_) => {},
                    Err(_) => {}
                }

            )*
        }
    };
}
