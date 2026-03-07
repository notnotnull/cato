use std::io::{self, Write};
use anyhow::{Context, Result};
use clap::{Parser};

/// Command-line Interface options
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number all output lines
    #[arg(short, long, default_value_t = false)]
    number: bool,
    /// file(s) to read
    #[arg(num_args = 0..)]
    files: Vec<std::path::PathBuf>,
}

/// Plain cato
fn simple_cato(args: Cli) -> Result<()> {
    // Get the global stdout entity
    let stdout = io::stdout();
    // wrap the handle in a buffer to reduce flushes
    let mut handle = io::BufWriter::new(stdout);

    for path in args.files {
        // Handle arguments with anyhow for more context
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Unable to read file `{}`", path.display()))?;

        for line in content.lines() {
            writeln!(handle, "{}", line)
                .with_context(|| format!("Unable to print file contents `{}`", path.display()))?;
        }
    }

    Ok(())
}

/// Option enabled cato
fn cato(args: Cli) -> Result<()> {
    // Get the global stdout entity
    let mut count: usize = 1;
    let stdout = io::stdout();
    // wrap the handle in a buffer to reduce flushes
    let mut handle = io::BufWriter::new(stdout);

    for path in args.files {
        // Handle arguments with anyhow for more context
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Unable to read file `{}`", path.display()))?;

        for line in content.lines() {
            writeln!(handle, "{:<5}{} {}","", count, line)
                .with_context(|| format!("Unable to print file contents `{}`", path.display()))?;
            count += 1;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // Handle arguments with clap
    let args = Cli::parse();

    if args.number == false {
        simple_cato(args).with_context(|| format!("simple_cato"))?;
    } else {
        cato(args).with_context(|| format!("cato"))?;
    }

    Ok(())
}
