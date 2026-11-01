use anyhow::bail;
use owo_colors::OwoColorize;
use serde_json::Value;

use crate::flags::Flags;

pub async fn make_query_params(f: &Flags) -> anyhow::Result<Vec<(String, String)>> {
    let mut params = Vec::<(String, String)>::new();

    for param in &f.query_params {
        if param.is_empty() {
            continue;
        }

        match param.split_once("=") {
            Some((k, v)) => params.push((k.to_string(), v.to_string())),
            None => match param.split_once(":") {
                Some((k, v)) => params.push((k.to_string(), v.to_string())),
                None => bail!(
                    "Invalid query parameter format (expected 'key:value' or 'key=value').".red()
                ),
            },
        }
    }

    Ok(params)
}

pub async fn make_headers(f: &Flags) -> anyhow::Result<reqwest::header::HeaderMap> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("rfetch/1.6.1"),
    );

    if f.json {
        let b = match &f.body {
            Some(s) => s,
            None => anyhow::bail!("Using the JSON feature requires a body to be set!"),
        };

        if let Err(e) = serde_json::from_str::<Value>(b) {
            bail!("{} {}", "JSON-Parse Error:".red(), e.red())
        };

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
    }

    for header in &f.headers {
        if header.is_empty() {
            continue;
        }

        let (k, v) = match header.split_once(':') {
            Some(kv) => kv,
            None => bail!("Invalid header format (expected 'key:value').".red()),
        };

        headers.insert(
            reqwest::header::HeaderName::from_bytes(k.as_bytes())?,
            reqwest::header::HeaderValue::from_bytes(v.as_bytes())?,
        );
    }

    Ok(headers)
}

pub async fn handle_response(r: reqwest::Response, f: &Flags) -> anyhow::Result<()> {
    if f.debug {
        println!("{:#?}", r);
        return Ok(());
    }

    if !f.body_only {
        println!("{}: {}", "Status".green().bold(), r.status().magenta());
        println!("{}:", "Headers".green().bold());
        for (k, v) in r.headers() {
            match v.to_str() {
                Ok(s) => println!("{}: {}", k.blue(), s.cyan()),
                Err(_) => continue,
            }
        }

        println!("\n{}", "Body:".green().bold());
    }

    if let Some(o) = &f.output {
        let b = r.bytes().await?;
        write_output_to_file(&b, o)?;
        println!("{} '{}'", "Wrote body to:".cyan(), o.purple());

        return Ok(());
    }

    let b = r.text().await?;
    if b == "" {
        println!("{}", "[ Empty Body ]".yellow());
    }

    println!("{b}");
    Ok(())
}

fn write_output_to_file(text: &[u8], path: &String) -> anyhow::Result<()> {
    if let Err(e) = std::fs::write(path, text) {
        anyhow::bail!("{} {}", "Error while writing to file:".red(), e.red())
    };

    Ok(())
}
