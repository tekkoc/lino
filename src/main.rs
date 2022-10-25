use argh::FromArgs;
use reqwest::{header, Client};
use std::collections::HashMap;

fn default_token_path() -> String {
    String::from("~/.lino_token")
}

#[derive(FromArgs, PartialEq, Debug)]
/// LINE Notify command.
struct Args {
    /// LINE Notify access token file path.
    #[argh(option, short = 't', default = "default_token_path()")]
    token: String,
    #[argh(positional)]
    message: Option<String>,
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    fn read_stdin() -> String {
        use std::io::stdin;
        use std::io::Read;

        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer).unwrap();
        buffer
    }

    let message = match args.message {
        Some(m) => m,
        None => read_stdin(),
    };

    send(message).await.unwrap();
}

async fn send(message: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = "Bearer [token]"; // TODO
    let url = "https://notify-api.line.me/api/notify";

    let mut params = HashMap::new();
    params.insert("message", &message);

    let mut head = header::HeaderMap::new();
    let token = header::HeaderValue::from_static(token);
    head.insert("Authorization", token);

    let client = Client::new();
    client.post(url).headers(head).form(&params).send().await?;

    Ok(())
}
