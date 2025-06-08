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
pub fn check_library_installation() -> Result<Option<YtdlpLibrary>, Error> {
    // If the library directory does not exist, create it.
    if !std::fs::exists("yt-dlp-libs")? {
        std::fs::create_dir("yt-dlp-libs")?
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
    let ffmpeg_installed: bool = std::fs::exists(FFMPEG_PATH)?;
    // Check if yt-dlp is installed.
    let yt_dlp_installed: bool = std::fs::exists(YTDLP_PATH)?;

    // If either library is missing: return which one.
    // If both are missing: return both.
    match (ffmpeg_installed, yt_dlp_installed) {
        (true, true) => Ok(None),
        (false, true) => Ok(Some(YtdlpLibrary::Ffmpeg)),
        (true, false) => Ok(Some(YtdlpLibrary::Ytdlp)),
        (false, false) => Ok(Some(YtdlpLibrary::Both))
    }
}

// Installs specified library/libraries from their respective GitHub source.
pub async fn install_libraries(missing_libraries: &YtdlpLibrary) -> Result<(), Error> {
    let installer = LibraryInstaller::new("yt-dlp-libs".into());

    match missing_libraries {
        YtdlpLibrary::Ffmpeg => {
            installer.install_ffmpeg(None).await?;
        }
        YtdlpLibrary::Ytdlp => {
            installer.install_youtube(None).await?;
        }
        YtdlpLibrary::Both => {
            installer.install_ffmpeg(None).await?;
            installer.install_youtube(None).await?;
        }
    }

    Ok(())
}
