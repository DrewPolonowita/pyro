use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    name: String,
    file: std::path::PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    println!("{}", args.name)
}