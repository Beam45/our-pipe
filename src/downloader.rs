use std::path::PathBuf;
use crate::utils;
use yt_dlp::{
    fetcher::deps,
    model::{
        AudioCodecPreference,
        AudioQuality,
        VideoCodecPreference,
        VideoQuality
    },
};

// Provides further abstraction over the yt-dlp library to create
// convenience methods to easily download YouTube videos based on format.
#[derive(Debug, Clone)]
pub struct Downloader {
    fetcher: yt_dlp::Youtube,
}

impl Downloader {
    // Output directory is the directory in which the downloaded content
    // will be placed.
    pub fn new(output_dir: Option<&str>) -> Result<Self, utils::Error> {
        // Collects required libraries from the library folder.
        let libraries =
            deps::Libraries::new("yt-dlp-libs/yt-dlp".into(), "yt-dlp-libs/ffmpeg".into());

        // If no output directory is specified, the default will
        // be set to 'yt-dlp-output'.
        let output_dir = match output_dir {
            Some(dir) => PathBuf::from(dir),
            None => PathBuf::from("yt-dlp-output"),
        };

        // Creates the yt-dlp video fetcher with the required libraries
        // and output directory.
        let fetcher = yt_dlp::Youtube::new(libraries, &output_dir)?;

        Ok(Self { fetcher })
    }

    pub async fn download_video_mp4(
        &self,
        url: &str,
        file_name: &str,
        video_quality: VideoQuality,
        audio_quality: AudioQuality,
    ) -> Result<PathBuf, utils::Error> {
        let output_path = self
            .fetcher
            .download_video_with_quality(
                url,
                format!("{file_name}.mp4"),
                video_quality,
                VideoCodecPreference::VP9,
                audio_quality,
                AudioCodecPreference::Opus,
            )
            .await?;

        Ok(output_path)
    }

    pub async fn download_video_mp3(
        &self,
        url: &str,
        file_name: &str,
        audio_quality: AudioQuality,
    ) -> Result<PathBuf, utils::Error> {
        let outpath_path = self
            .fetcher
            .download_audio_stream_with_quality(
                url,
                format!("{file_name}.mp3"),
                audio_quality,
                AudioCodecPreference::Opus,
            )
            .await?;

        Ok(outpath_path)
    }
}
