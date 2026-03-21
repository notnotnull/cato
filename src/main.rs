use std::io::{self, Read, Write};
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

fn read_stdin() -> Result<String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .with_context(|| "Failed to read from stdin")?;
    Ok(input)
}

/// Plain cato
fn simple_cato(args: Cli) -> Result<()> {
    // Get the global stdout entity
    let stdout = io::stdout();
    // wrap the handle in a buffer to reduce flushes
    let mut handle = io::BufWriter::new(stdout);

    let files = if args.files.is_empty() {
        vec![std::path::PathBuf::from("-")]
    } else {
        args.files
    };

    for path in files {
        if path == std::path::PathBuf::from("-") {
            let input = read_stdin()?;
            for line in input.lines() {
                writeln!(handle, "{}", line)
                    .with_context(|| "Unable to print stdin contents")?;
            }
            continue;
        }
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

    let files = if args.files.is_empty() {
        vec![std::path::PathBuf::from("-")]
    } else {
        args.files
    };

    for path in files {
        if path == std::path::PathBuf::from("-") {
            let input = read_stdin()?;
            for line in input.lines() {
                writeln!(handle, "{:<4}{}{:<2}{}","", count, "", line)
                    .with_context(|| "Unable to print stdin contents")?;
                count += 1;
            }
            continue;
        }
        // Handle arguments with anyhow for more context
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Unable to read file `{}`", path.display()))?;

        for line in content.lines() {
            writeln!(handle, "{:<4}{}{:<2}{}","", count, "", line)
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
