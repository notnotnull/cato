use clap::Parser;

#[derive(Parser)]
struct Cli {
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    // Handle arguments with clap
    let args = Cli::parse();

    println!("path: {:?}", args.path)
}
