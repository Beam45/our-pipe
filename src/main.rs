mod app;
mod downloader;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = app::SlintApp::new()?;

    app.run().await?;

    Ok(())
}
