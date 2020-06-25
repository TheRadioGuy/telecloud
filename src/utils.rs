#[macro_export]
macro_rules! create_dir_if_dont_exist {
    ( $( $x:expr ),* ) => {
        async move {
            use tokio::fs::create_dir;
            $(
                create_dir($x).await;
            )*
        }
    };
}