use yt_dlp::fetcher::deps::LibraryInstaller;

pub enum YtdlpLibrary {
    Ffmpeg,
    Ytdlp,
    Both,
}

// check if required yt_dlp library binaries are installed
// return which libraries need to be installed
pub async fn check_library_installation() -> Result<Option<YtdlpLibrary>, anyhow::Error> {
    // if library directory does not exist, create it
    if !tokio::fs::try_exists("yt-dlp-libs").await? {
        tokio::fs::create_dir("yt-dlp-libs").await?
    }

    // check if ffmpeg is installed
    let ffmpeg_installed: bool = tokio::fs::try_exists("yt-dlp-libs/ffmpeg").await?;

    // check if yt-dlp is installed
    let yt_dlp_installed: bool = tokio::fs::try_exists("yt-dlp-libs/yt-dlp").await?;

    // if either library is missing: return which one
    // if both are missing: return both
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

pub async fn install_libraries(library: YtdlpLibrary) -> Result<(), anyhow::Error> {
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
