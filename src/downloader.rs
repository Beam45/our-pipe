use std::path::PathBuf;
use yt_dlp::{
    fetcher::deps,
    model::{AudioCodecPreference, AudioQuality, VideoCodecPreference, VideoQuality},
};

pub struct Downloader {
    fetcher: yt_dlp::Youtube,
}

impl Downloader {
    pub fn new(output_dir: Option<&str>) -> Result<Self, anyhow::Error> {
        let libraries =
            deps::Libraries::new("yt-dlp-libs/yt-dlp".into(), "yt-dlp-libs/ffmpeg".into());

        let output_dir = match output_dir {
            Some(dir) => PathBuf::from(dir),
            None => PathBuf::from("yt-dlp-output"),
        };

        let fetcher = yt_dlp::Youtube::new(libraries, &output_dir)?;

        Ok(Self { fetcher })
    }

    pub async fn download_video_mp4(
        &mut self,
        url: &str,
        file_name: &str,
        video_quality: VideoQuality,
        audio_quality: AudioQuality,
    ) -> Result<PathBuf, anyhow::Error> {
        let output_path = self
            .fetcher
            .download_video_with_quality(
                url,
                format!("{file_name}.mp4"),
                video_quality,
                VideoCodecPreference::Any,
                audio_quality,
                AudioCodecPreference::Any,
            )
            .await?;

        Ok(output_path)
    }

    pub async fn download_video_mp3(
        &mut self,
        url: &str,
        file_name: &str,
        quality: AudioQuality,
    ) -> Result<PathBuf, anyhow::Error> {
        let outpath_path = self
            .fetcher
            .download_audio_stream_with_quality(
                url,
                format!("{file_name}.mp3"),
                quality,
                AudioCodecPreference::Any,
            )
            .await?;

        Ok(outpath_path)
    }
}
