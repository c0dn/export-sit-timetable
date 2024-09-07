use crate::models::GithubLatestReleaseRes;

pub async fn get_latest_release() -> Result<String, String> {
    let r = reqwest::get("https://api.github.com/repos/c0dn/export-sit-timetable/releases/latest")
        .await
        .map_err(|_| "Version check failed: Network error")?;
    let latest_version = r
        .json::<GithubLatestReleaseRes>()
        .await
        .map_err(|_| "Version check failed: JSON error")?;
    Ok(latest_version.tag_name)
}