pub fn get_contributors(url: &str) -> Result<String, std::io::Error> {
    match ureq::get(url)
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .call() {
        Ok(response) => {
            Ok(response.into_string()?)
        }
        Err(e) => {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Could not load contributors from api {} with error: {}", url, e.to_string()),
            ))
        }
    }
}
