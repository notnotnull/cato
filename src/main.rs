use anyhow::{Context, Result};
use clap::Parser;

/// Read a file
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // Handle arguments with clap
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Unable to read file `{}`", args.path.display()))?;

    for line in content.lines() {
        println!("{}", line);
    }

    Ok(())
}
