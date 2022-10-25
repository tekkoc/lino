use argh::FromArgs;

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

fn main() {
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

    dbg!(message);
}
