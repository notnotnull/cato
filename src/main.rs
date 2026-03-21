use std::io::{self, Read, Write};
use anyhow::{Context, Result};
use clap::{Parser};

/// Command-line Interface options
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank", default_value_t = false)]
    number_nonblank: bool,
    /// Number all output lines
    #[arg(short = 'n', long, default_value_t = false)]
    number: bool,
    /// file(s) to read
    #[arg(num_args = 0..)]
    files: Vec<std::path::PathBuf>,
}

#[derive(Copy, Clone)]
enum NumberMode {
    None,
    All,
    NonBlank,
}

fn read_stdin() -> Result<String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .with_context(|| "Failed to read from stdin")?;
    Ok(input)
}

fn cato(args: Cli, mode: NumberMode) -> Result<()> {
    let mut count: usize = 1;
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let files = if args.files.is_empty() {
        vec![std::path::PathBuf::from("-")]
    } else {
        args.files
    };

    for path in files {
        let content = if path == std::path::PathBuf::from("-") {
            read_stdin()?
        } else {
            std::fs::read_to_string(&path)
                .with_context(|| format!("Unable to read file {}", path.display()))?
        };

        for line in content.lines() {
            match mode {
                NumberMode::None => {
                    writeln!(handle, "{}", line)
                        .with_context(|| "Unable to print contents")?;
                }
                NumberMode::All => {
                    writeln!(handle, "{:<4}{}{:<2}{}", "", count, "", line)
                        .with_context(|| "Unable to print contents")?;
                    count += 1;
                }
                NumberMode::NonBlank => {
                    if line.is_empty() {
                        writeln!(handle)
                            .with_context(|| "Unable to print contents")?;
                    } else {
                        writeln!(handle, "{:<4}{}{:<2}{}", "", count, "", line)
                            .with_context(|| "Unable to print contents")?;
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // Handle arguments with clap
    let args = Cli::parse();

    let mode = if args.number_nonblank {
        NumberMode::NonBlank
    } else if args.number {
        NumberMode::All
    } else {
        NumberMode::None
    };

    cato(args, mode).with_context(|| format!("cato"))?;

    Ok(())
}
