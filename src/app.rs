use crate::utils;
use crate::downloader::Downloader;
use yt_dlp::model::{AudioQuality, VideoQuality};

slint::include_modules!();

#[derive(Debug, Clone)]
enum Format {
    Mp4,
    Mp3,
}

#[derive(Debug, Clone)]
enum Quality {
    Best,
    High,
    Medium,
    Low,
    Worst,
}

pub struct SlintApp {
    downloader: Downloader,
    url: String,
    file_name: String,
    format: Format,
    quality: Quality,
}

impl SlintApp {
    pub fn new() -> Result<Self, utils::Error> {
        Ok(Self {
            downloader: Downloader::new(None)?,
            url: String::new(),
            file_name: String::new(),
            format: Format::Mp4,
            quality: Quality::Best,
        })
    }

    pub async fn run(&mut self) -> Result<(), utils::Error> {
        if let Some(missing_libs) = utils::check_library_installation().await? {
            println!("missing library/libraries: {missing_libs:?}");

            if Self::show_download_lib_dialog() == rfd::MessageDialogResult::Ok {
                if let Err(lib_download_error) = utils::install_libraries(missing_libs).await {
                    Self::show_error_dialog(&format!("{lib_download_error}")).await;
                }
            } else {
                return Ok(());
            }
        }

        

        Ok(())
    }

    async fn download_video(&self) -> Result<(), utils::Error> {
        let (video_quality, audio_quality) = match self.quality {
            Quality::Best => (VideoQuality::Best, AudioQuality::Best),
            Quality::High => (VideoQuality::High, AudioQuality::High),
            Quality::Medium => (VideoQuality::Medium, AudioQuality::Medium),
            Quality::Low => (VideoQuality::Low, AudioQuality::Low),
            Quality::Worst => (VideoQuality::Worst, AudioQuality::Worst),
        };

        match self.format {
            Format::Mp4 => {
                self.downloader.download_video_mp4(
                    &self.url,
                    &self.file_name,
                    video_quality,
                    audio_quality
                ).await?;
            }
            Format::Mp3 => {
                self.downloader.download_video_mp3(
                    &self.url,
                    &self.file_name,
                    audio_quality
                ).await?;
            }
        }

        Ok(())
    }

    fn show_download_lib_dialog() -> rfd::MessageDialogResult {
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Info)
            .set_title("Installing Libraries")
            .set_description("Required libraries not found, click ok to install them")
            .set_buttons(rfd::MessageButtons::Ok)
            .show()
    }

    async fn show_error_dialog(error_msg: &str) {
        rfd::AsyncMessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Error")
            .set_description(format!("An error has occurred: {error_msg}"))
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
}
