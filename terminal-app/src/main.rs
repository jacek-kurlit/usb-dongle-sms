use clap::{arg, command, Parser};

fn main() {
    let args = Args::parse();

    println!("Hello {}", args.message)
}

#[derive(Parser, Debug)]
#[command(name = "sms", author = "J.Kurlit", about, version)]
struct Args {
    #[arg(short, long)]
    message: String,
}
