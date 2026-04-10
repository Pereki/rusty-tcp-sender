use clap::Parser;

#[derive(Parser, Debug)]
pub struct Argument{

    /// The Message you want to send
    #[arg(long)]
    pub send: Option<String>,

    /// If you want to receive a message
    #[arg(long)]
    pub receive: bool,

    /// Where to connect (sender) or bind (receiver)
    #[arg(long, default_value = "127.0.0.1:9001")]
    pub addr: String,
}