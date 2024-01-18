use colored::*;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::io::Write;
use std::{fs::OpenOptions, time::Duration};

use crate::utils::hasher;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/115.0";

pub async fn visit(
    trigger_word: Option<String>,
    ip: String,
    output_path: Option<String>,
    favicon_hash: Option<String>,
    host: String,
    https: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        HeaderValue::from_str(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        )?,
    );
    // headers.insert(
    //     "Accept-Language
    //     ",
    //     HeaderValue::from_static("en-US,en;q=0.5"),
    // );
    headers.insert("User-Agent", HeaderValue::from_str(USER_AGENT)?);
    headers.insert("host", HeaderValue::from_str(&host)?);
    headers.insert("Connection", HeaderValue::from_str("keep-alive")?);

    if trigger_word.is_some() {
        let url = if https {
            format!("https://{}", ip)
        } else {
            format!("http://{}", ip)
        };

        let res = client.get(url).headers(headers.clone()).send().await?;

        let text = res.text().await?;

        if text.contains(&trigger_word.unwrap()) {
            match output_path {
                Some(path) => {
                    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

                    writeln!(file, "{}", ip)?;
                }
                None => {
                    println!("{}: {}", "Found".green(), ip);
                }
            }

            return Ok(true);
        }
    }

    if favicon_hash.is_some() {
        let url = if https {
            format!("https://{}/favicon.ico", ip)
        } else {
            format!("http://{}/favicon.ico", ip)
        };

        let res = client.get(url).headers(headers).send().await?;

        let octet = res.bytes().await?;

        let hash = hasher(octet);

        if favicon_hash.unwrap() == hash {
            match output_path {
                Some(path) => {
                    let mut file = OpenOptions::new().append(true).open(path)?;

                    writeln!(file, "{}", ip)?;
                }
                None => {
                    println!("{}: {}", "Found".green(), ip);
                }
            }

            return Ok(true);
        }
    }

    Ok(false)
}
