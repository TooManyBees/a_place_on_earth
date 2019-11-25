use std::collections::HashMap;

struct Keys {
    consumer_key: String,
    secret_key: String,
    access_token: String,
    secret_token: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = heaven::random_post();
    println!("{}", message);

    let mut request_url = url::Url::parse("https://api.twitter.com/1.1/statuses/update.json")?;
    request_url
        .query_pairs_mut()
        .append_pair("status", &message);

    let keys = {
        dotenv::dotenv()?;
        let mut missing_env_vars = vec![];
        let mut env_vars: HashMap<&'static str, String> = [
            "PLACE_ON_EARTH_CONSUMER_KEY",
            "PLACE_ON_EARTH_SECRET_KEY",
            "PLACE_ON_EARTH_ACCESS_TOKEN",
            "PLACE_ON_EARTH_SECRET_ACCESS_TOKEN",
        ]
        .iter()
        .filter_map(|&key| match std::env::var(key) {
            Ok(s) if !s.is_empty() => Some((key, s)),
            _ => {
                missing_env_vars.push(key);
                None
            }
        })
        .collect();

        if !missing_env_vars.is_empty() {
            eprintln!(
                "Missing required env vars:\n  {}",
                missing_env_vars.join("\n  ")
            );
            std::process::exit(1);
        }

        Keys {
            consumer_key: env_vars.remove("PLACE_ON_EARTH_CONSUMER_KEY").unwrap(),
            secret_key: env_vars.remove("PLACE_ON_EARTH_SECRET_KEY").unwrap(),
            access_token: env_vars.remove("PLACE_ON_EARTH_ACCESS_TOKEN").unwrap(),
            secret_token: env_vars
                .remove("PLACE_ON_EARTH_SECRET_ACCESS_TOKEN")
                .unwrap(),
        }
    };

    let header = oauthcli::OAuthAuthorizationHeaderBuilder::new(
        "POST",
        &request_url,
        keys.consumer_key,
        keys.secret_key,
        oauthcli::SignatureMethod::HmacSha1,
    )
    .token(keys.access_token, keys.secret_token)
    .finish_for_twitter();

    let client = reqwest::Client::new();
    let mut res = client
        .post(request_url.as_str())
        .header("Authorization", header.to_string())
        .send()?;
    dbg!(&res);
    if res.status().is_success() {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
