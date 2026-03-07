use std::io::{self, Write};
use anyhow::{Context, Result};
use clap::Parser;

/// Command-line Interface options
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // Handle arguments with clap
    let args = Cli::parse();

    // Handle arguments with anyhow for more context
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Unable to read file `{}`", args.path.display()))?;

    // Get the global stdout entity
    let stdout = io::stdout();
    // wrap the handle in a buffer to reduce flushes
    let mut handle = io::BufWriter::new(stdout);

    for line in content.lines() {
        writeln!(handle, "{}", line)
            .with_context(|| format!("Unable to print file contents `{}`", args.path.display()))?;
    }

    Ok(())
}
