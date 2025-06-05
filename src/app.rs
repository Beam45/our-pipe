use crate::downloader::Downloader;
use yt_dlp::model::{AudioQuality, VideoQuality};

slint::include_modules!();

pub struct SlintApp {
    main_window: MainWindow,
}

impl SlintApp {
    pub fn new() -> anyhow::Result<Self> {
        let main_window = MainWindow::new()?;

        Ok(Self { main_window })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        if let Some(missing_libs) = crate::utils::check_library_installation().await? {
            println!("detected missing libraries, downloading now...");
            crate::utils::install_libraries(missing_libs).await?;
        }

        self.main_window.on_download_button_clicked(move |url, file_name, format, quality| {
            println!("download button clicked...");
            println!("video: url: {url}, file name: {file_name}, format: {format}, quality: {quality}");

            println!("starting video download...");
            slint::spawn_local(async move {
                match Downloader::new(None) {
                    Ok(mut downloader) => {
                        tokio::spawn(async move {
                            println!("downloading video...");
                            match Self::download_video(&mut downloader, &url, &file_name, &format, &quality).await {
                                Ok(()) => {
                                    println!("video successfully downloaded!");
                                    Self::show_success_dialog().await;
                                }
                                Err(download_error) => {
                                    println!("error while downloading: {download_error}");
                                    Self::show_error_dialog(&format!("{download_error}")).await;
                                }
                            }
                        });
                    }
                    Err(downloader_error) => {
                        println!("downloader error: {downloader_error}");
                        Self::show_error_dialog(&format!("{downloader_error}")).await;
                    }
                }
            }).unwrap();
        });

        self.main_window.run()?;

        Ok(())
    }

    async fn download_video(
        downloader: &mut Downloader,
        url: &str,
        file_name: &str,
        format: &str,
        quality: &str,
    ) -> Result<(), anyhow::Error> {
        let (video_quality, audio_quality) = match quality {
            "Best" => (VideoQuality::Best, AudioQuality::Best),
            "High" => (VideoQuality::High, AudioQuality::High),
            "Medium" => (VideoQuality::Medium, AudioQuality::Medium),
            "Low" => (VideoQuality::Low, AudioQuality::Low),
            "Worst" => (VideoQuality::Worst, AudioQuality::Worst),
            _ => (VideoQuality::Best, AudioQuality::Best),
        };

        match format {
            "mp4" => {
                downloader
                    .download_video_mp4(url, file_name, video_quality, audio_quality)
                    .await?;
            }
            "mp3" => {
                downloader
                    .download_video_mp3(url, file_name, audio_quality)
                    .await?;
            }
            _ => (),
        }

        Ok(())
    }

    async fn show_error_dialog(error_msg: &str) {
        rfd::AsyncMessageDialog::new()
            .set_level(rfd::MessageLevel::Warning)
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
