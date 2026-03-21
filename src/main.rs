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
    /// Display $ at the end of each line
    #[arg(short = 'E', long = "show-ends", default_value_t = false)]
    show_ends: bool,
    /// Number all output lines
    #[arg(short = 'n', long, default_value_t = false)]
    number: bool,
    /// Suppress repeated empty output lines
    #[arg(short = 's', long = "squeeze-blank", default_value_t = false)]
    squeeze_blank: bool,
    /// Display TAB characters as ^I
    #[arg(short = 'T', long = "show-tabs", default_value_t = false)]
    show_tabs: bool,
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
    let mut squeeze_count: usize = 0;
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let files = if args.files.is_empty() {
        vec![std::path::PathBuf::from("-")]
    } else {
        args.files
    };

    for path in files {
        let content = if path.to_str() == Some("-") {
            read_stdin()?
        } else {
            std::fs::read_to_string(&path)
                .with_context(|| format!("Unable to read file {}", path.display()))?
        };

        for raw_line in content.split_inclusive('\n') {
            let has_newline = raw_line.ends_with('\n');
            let line = if has_newline {
                &raw_line[..raw_line.len() - 1]
            } else {
                raw_line
            };

            if line.is_empty() && args.squeeze_blank {
                squeeze_count += 1;
                if squeeze_count > 1 {
                    continue;
                }
            } else {
                squeeze_count = 0;
            }


            let mut rendered_line = if args.show_ends {
                format!("{}$", line)
            } else {
                line.to_string()
            };

            if args.show_tabs {
                rendered_line = rendered_line.replace("\t", "^I");
            }

            match mode {
                NumberMode::None => {
                    write!(handle, "{}", rendered_line)
                        .with_context(|| "Unable to print contents")?;
                }
                NumberMode::All => {
                    write!(handle, "{:<4}{}{:<2}{}", "", count, "", rendered_line)
                        .with_context(|| "Unable to print contents")?;
                    count += 1;
                }
                NumberMode::NonBlank => {
                    if line.is_empty() {
                        if args.show_ends {
                            write!(handle, "$")
                                .with_context(|| "Unable to print contents")?;
                        } else {
                            write!(handle, "")
                                .with_context(|| "Unable to print contents")?;
                        }
                    } else {
                        write!(handle, "{:<4}{}{:<2}{}", "", count, "", rendered_line)
                            .with_context(|| "Unable to print contents")?;
                        count += 1;
                    }
                }
            }

            if has_newline {
                writeln!(handle).with_context(|| "Unable to print contents")?;
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

    cato(args, mode).with_context(|| "cato")?;

    Ok(())
}
