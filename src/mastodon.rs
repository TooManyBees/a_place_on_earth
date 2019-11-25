use reqwest::{Client, StatusCode};

pub fn send(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("PLACE_ON_EARTH_BOTSIN_SPACE_TOKEN")?;

    let request_url = {
        let mut request_url = url::Url::parse("https://botsin.space/api/v1/statuses")?;
        request_url
            .query_pairs_mut()
            .append_pair("status", &message);
        request_url.into_string().replace("+", "%20")
    };

    let client = Client::new();
    let res = client
        .post(&request_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?;

    dbg!(res);
    Ok(())
}
