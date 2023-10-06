use log::LevelFilter;

use yeah::run;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    run().await;
}
