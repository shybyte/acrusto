use open;

pub fn open_url(url: &str) -> Result<std::process::ExitStatus, String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        open::that(url).map_err(|e| format!("Can't open URL \"{:?}\" because of {}", url, e))
    } else {
        Err(format!("Can't open invalid URL \"{}\"", url))
    }
}