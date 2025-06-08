#![windows_subsystem = "windows"]

mod app;
mod downloader;
mod utils;

#[tokio::main]
async fn main() -> Result<(), crate::utils::Error> {
    app::run_app().await?;

    Ok(())
}
