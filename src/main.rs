use tokio::runtime::Builder;

fn main() {
    Builder::new()
        .enable_all()
        .threaded_scheduler()
        .build()
        .expect("failed to initialize tokio runtime")
        .block_on(async_main());
}

async fn async_main() {
    println!("Hello World")
}
