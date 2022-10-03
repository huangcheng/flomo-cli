use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Post a memo
    pub memo: Option<String>,

    /// Set token
    #[arg(short, long, value_name = "TOKEN")]
    pub config: Option<String>,
}