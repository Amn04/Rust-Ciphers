use clap::{command, Parser};
use clap_derive::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[derive(Debug)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Cli::parse();
    
}
