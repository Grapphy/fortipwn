extern crate reqwest;

pub async fn make_query(
    token: &str,
    query: &str,
    page: u64,
) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!(
        "https://api.shodan.io/shodan/host/search?key={}&query={}&page={}",
        token, query, page
    );
    let result = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    Ok(result)
}
