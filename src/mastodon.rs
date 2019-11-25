pub async fn send(message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let access_token = std::env::var("PLACE_ON_EARTH_BOTSIN_SPACE_TOKEN")?;

    let request_url = {
        let mut request_url = url::Url::parse("https://botsin.space/api/v1/statuses")?;
        request_url
            .query_pairs_mut()
            .append_pair("status", &message);
        request_url.into_string().replace("+", "%20")
    };

    let res = surf::post(&request_url)
        .set_header("Authorization", format!("Bearer {}", access_token))
        .await?;
    Ok(())
}
