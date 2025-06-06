use yt_dlp::fetcher::deps::LibraryInstaller;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("yt-dlp error: {0}")]
    YtdlpError(#[from] yt_dlp::error::Error),
    #[error("slint platform error: {0}")]
    SlintPlatformError(#[from] slint::PlatformError),
}

#[derive(Debug)]
pub enum YtdlpLibrary {
    Ffmpeg,
    Ytdlp,
    Both,
}

// Check if required yt_dlp library binaries are installed,
// return which libraries need to be installed.
pub async fn check_library_installation() -> Result<Option<YtdlpLibrary>, Error> {
    // If library directory does not exist, create it.
    if !tokio::fs::try_exists("yt-dlp-libs").await? {
        tokio::fs::create_dir("yt-dlp-libs").await?
    }

    // Library installation path for windows.
    #[cfg(target_os = "windows")]
    const FFMPEG_PATH: &str = "yt-dlp-libs/ffmpeg.exe";
    #[cfg(target_os = "windows")]
    const YTDLP_PATH: &str = "yt-dlp-libs/yt-dlp.exe";

    // Library installation path for linux.
    #[cfg(target_os = "linux")]
    const FFMPEG_PATH: &str = "yt-dlp-libs/ffmpeg";
    #[cfg(target_os = "linux")]
    const YTDLP_PATH: &str = "yt-dlp-libs/yt-dlp";

    // Check if ffmpeg is installed.
    let ffmpeg_installed: bool = tokio::fs::try_exists(FFMPEG_PATH).await?;
    // Check if yt-dlp is installed.
    let yt_dlp_installed: bool = tokio::fs::try_exists(YTDLP_PATH).await?;

    // If either library is missing: return which one,
    // if both are missing: return both.
    if !ffmpeg_installed {
        return Ok(Some(YtdlpLibrary::Ffmpeg));
    } else if !yt_dlp_installed {
        return Ok(Some(YtdlpLibrary::Ytdlp));
    } else if !ffmpeg_installed && !yt_dlp_installed {
        return Ok(Some(YtdlpLibrary::Both));
    } else {
        Ok(None)
    }
}

// Installs specified library/libraries from github source.
pub async fn install_libraries(library: YtdlpLibrary) -> Result<(), Error> {
    match library {
        YtdlpLibrary::Ffmpeg => {
            LibraryInstaller::new("yt-dlp-libs".into())
                .install_ffmpeg(None)
                .await?;
        }
        YtdlpLibrary::Ytdlp => {
            LibraryInstaller::new("yt-dlp-libs".into())
                .install_youtube(None)
                .await?;
        }
        YtdlpLibrary::Both => {
            LibraryInstaller::new("yt-dlp-libs".into())
                .install_ffmpeg(None)
                .await?;
            LibraryInstaller::new("yt-dlp-libs".into())
                .install_youtube(None)
                .await?;
        }
    }

    Ok(())
}
