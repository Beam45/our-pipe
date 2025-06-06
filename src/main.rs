mod app;
mod downloader;
mod utils;

#[tokio::main]
async fn main() -> Result<(), crate::utils::Error> {
    let mut app = app::SlintApp::new()?;

    app.run().await?;

    Ok(())
}
