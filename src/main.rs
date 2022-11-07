use anyhow::anyhow;
use argh::FromArgs;
use reqwest::{header, Client};
use std::collections::HashMap;

fn default_token_path() -> String {
    String::from("~/.lino_token")
}

#[derive(FromArgs, PartialEq, Debug)]
/// LINE Notify command.
struct Args {
    /// LINE Notify access token file path (default="~/.lino_token").
    #[argh(option, short = 'f', default = "default_token_path()")]
    file: String,
    /// LINE Notify access token.
    #[argh(option, short = 't')]
    token: Option<String>,
    #[argh(positional)]
    message: Option<String>,
}

fn read_stdin() -> String {
    use std::io::stdin;
    use std::io::Read;

    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn read_file(path: String) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let path: String = shellexpand::tilde(&path).into();

    let mut f = File::open(path)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let content = content.trim().to_string();

    Ok(content)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    let message = match args.message {
        Some(m) => m,
        None => read_stdin(),
    };

    let token = match (args.token, args.file) {
        (Some(t), _) => t,
        (None, file) => read_file(file)?,
    };

    send(message, token).await?;

    Ok(())
}

async fn send(message: String, token: String) -> anyhow::Result<()> {
    // referred to https://zenn.dev/alfina2538/articles/b2de12cdbbf30a

    let token = format!("Bearer {}", token.clone());
    let url = "https://notify-api.line.me/api/notify";

    let mut params = HashMap::new();
    params.insert("message", &message);

    let mut head = header::HeaderMap::new();
    let token = header::HeaderValue::from_str(&token)?;
    head.insert("Authorization", token);

    let client = Client::new();
    let response = client.post(url).headers(head).form(&params).send().await?;

    match response.status() {
        s if s.is_success() => Ok(()),
        s if s.is_server_error() => Err(anyhow!("server error")),
        _ => Err(anyhow!("client error")),
    }
}
