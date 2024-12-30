use crate::models::GithubLatestReleaseRes;

pub async fn get_latest_release() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .user_agent("export-sit-timetable/1.1.0")
        .build().unwrap();
    let r = client.get("https://api.github.com/repos/c0dn/export-sit-timetable/releases/latest")
        .send()
        .await
        .map_err(|_| "Version check failed: Network error")?;
    let latest_version = r
        .json::<GithubLatestReleaseRes>()
        .await
        .map_err(|_| "Version check failed: JSON error")?;
    Ok(latest_version.tag_name)
}
