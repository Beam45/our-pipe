use crate::utils;
use crate::downloader::Downloader;
use yt_dlp::model::{AudioQuality, VideoQuality};

slint::include_modules!();

pub async fn run_app() -> Result<(), utils::Error> {
    if let Some(missing_libs) = utils::check_library_installation()? {
        show_download_lib_dialog(&missing_libs).await;
        utils::install_libraries(&missing_libs).await?;
    }

    let window = MainWindow::new()?;

    window.on_download_button_clicked(begin_download);

    window.run()?;

    Ok(())
}

fn begin_download(
    url: slint::SharedString,
    file_name: slint::SharedString,
    format: slint::SharedString,
    quality: slint::SharedString
) {
    slint::spawn_local(async move {
        let downloader = Downloader::new(None).unwrap();

        let url = url.as_str();
        let file_name = file_name.as_str();
        let (video_quality, audio_quality) = match quality.as_str() {
            "Best" => (VideoQuality::Best, AudioQuality::Best),
            "High" => (VideoQuality::High, AudioQuality::High),
            "Medium" => (VideoQuality::Medium, AudioQuality::Medium),
            "Low" => (VideoQuality::Low, AudioQuality::Low),
            "Worst" => (VideoQuality::Worst, AudioQuality::Worst),
            _ => (VideoQuality::Best, AudioQuality::Best)
        };

        match format.as_str() {
            "mp4" => {
                show_downloading_dialog().await;
                match downloader.download_video_mp4(
                    url,
                    file_name,
                    video_quality,
                    audio_quality
                ).await {
                    Ok(_) => show_success_dialog().await,
                    Err(download_err) => show_error_dialog(format!("{download_err}")).await
                }
            }
            "mp3" => {
                show_downloading_dialog().await;
                match downloader.download_video_mp3(
                    url,
                    file_name,
                    audio_quality
                ).await {
                    Ok(_) => show_success_dialog().await,
                    Err(download_err) => show_error_dialog(format!("{download_err}")).await
                }
            }
            _ => {}
        }
    }).unwrap();
}

async fn show_error_dialog(error_msg: String) {
    rfd::AsyncMessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Error")
        .set_description(format!("An error has occurred: {error_msg}"))
        .show()
        .await;
}

async fn show_downloading_dialog() {
    rfd::AsyncMessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_title("Starting Download")
        .set_description("Video download has begun.")
        .set_buttons(rfd::MessageButtons::Ok)
        .show()
        .await;
}

async fn show_success_dialog() {
    rfd::AsyncMessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_title("Success!")
        .set_description("Video was downloaded successfully!")
        .set_buttons(rfd::MessageButtons::Ok)
        .show()
        .await;
}

async fn show_download_lib_dialog(missing_lib: &utils::YtdlpLibrary) {
    let missing_lib_text = match missing_lib {
        utils::YtdlpLibrary::Ffmpeg => "FFMPEG not found, installing it now...",
        utils::YtdlpLibrary::Ytdlp => "yt-dlp not found, installing it now...",
        utils::YtdlpLibrary::Both => "FFPMPEG and yt-dlp not found, installing them now..."
    };

    rfd::AsyncMessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_title("Installing Libraries")
        .set_description(missing_lib_text)
        .set_buttons(rfd::MessageButtons::Ok)
        .show()
        .await;
}
