use tokio::process::Command;
pub async fn yt_dlp(url: String, path: String) -> std::io::Result<String> {
    dbg!(&url, &path);
    Command::new("yt-dlp")
        .args(&[url, "--output".into(), path.clone()])
        .output()
        .await?;
    Ok(path)
}
