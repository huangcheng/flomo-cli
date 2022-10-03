mod cli;
mod config;
mod api;

use clap::Parser;

use cli::Args;
use config::{save, read};
use api::{send};

fn main() {
    let args = Args::parse();

    if let Some(token) = args.config.as_deref() {
        save(token)
    }

    if let Some(memo) = args.memo.as_deref() {
        let token = match read() {
            Ok(token) => token,
            Err(_) => panic!("Failed to read token"),
        };

        send(token.as_str(), memo);
    }
}
