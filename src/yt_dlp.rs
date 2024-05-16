use tokio::process::Command;
async fn retry_dl(url: String, path: String, mut count: usize) -> std::io::Result<String> {
    while count > 0 {
        match Command::new("yt-dlp")
            .args(&[url.clone(), "--output".into(), path.clone()])
            .output()
            .await
        {
            Ok(_) => return Ok(path),
            Err(e) => {
                if count == 0 {
                    return Err(e);
                } else {
                    count -= 1;
                    continue;
                }
            }
        }
    }
    panic!("unreachable?")
}
pub async fn yt_dlp(url: String, path: String) -> std::io::Result<String> {
    dbg!(&url, &path);
    retry_dl(url, path, 3).await
}
