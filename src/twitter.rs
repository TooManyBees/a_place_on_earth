use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use surf::http::status::StatusCode;

struct Keys {
    consumer_key: String,
    secret_key: String,
    access_token: String,
    secret_token: String,
}

#[derive(Debug)]
pub enum TwitterError {
    MissingEnvVars(Vec<&'static str>),
    UpdateError(StatusCode, Vec<TwitterResponseError>),
}

impl Display for TwitterError {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            TwitterError::MissingEnvVars(vars) => write!(
                formatter,
                "Can't post to Twitter. Missing required env vars:\n  {}",
                vars.join("\n  ")
            ),
            TwitterError::UpdateError(status, errors) => {
                let errors = errors
                    .iter()
                    .map(|e| format!("Code {}: \"{}\"", e.code, e.message))
                    .collect::<Vec<String>>()
                    .join("\n  ");
                write!(
                    formatter,
                    "Can't post to Twitter. HTTP status {}:\n  {}",
                    status, errors,
                )
            }
        }
    }
}

impl Error for TwitterError {}

#[derive(Deserialize)]
struct TwitterResponseErrors {
    errors: Vec<TwitterResponseError>,
}
#[derive(Debug, Deserialize)]
pub struct TwitterResponseError {
    code: u32,
    message: String,
}

pub async fn send(message: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let request_url = {
        let mut request_url = url::Url::parse("https://api.twitter.com/1.1/statuses/update.json")?;
        request_url
            .query_pairs_mut()
            .append_pair("status", &message);
        url::Url::parse(&request_url.into_string().replace("+", "%20"))?
    };

    let keys = {
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
            Err(TwitterError::MissingEnvVars(missing_env_vars))?;
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

    let mut res = surf::post(request_url)
        .set_header("Authorization", header.to_string())
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let err: TwitterResponseErrors = res.body_json().await?;
        Err(TwitterError::UpdateError(res.status(), err.errors))?
    }
}
