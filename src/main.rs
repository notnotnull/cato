use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use anyhow::{Context, Result};
use clap::Parser;

/// Command-line Interface options
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank", default_value_t = false)]
    number_nonblank: bool,
    /// Equivalent to -vE
    #[arg(short = 'e', default_value_t = false)]
    combination_e: bool,
    /// Display $ at the end of each line
    #[arg(short = 'E', long = "show-ends", default_value_t = false)]
    show_ends: bool,
    /// Number all output lines
    #[arg(short = 'n', long, default_value_t = false)]
    number: bool,
    /// Suppress repeated empty output lines
    #[arg(short = 's', long = "squeeze-blank", default_value_t = false)]
    squeeze_blank: bool,
    /// Equivalent to -vT
    #[arg(short = 't', default_value_t = false)]
    combination_t: bool,
    /// Display TAB characters as ^I
    #[arg(short = 'T', long = "show-tabs", default_value_t = false)]
    show_tabs: bool,
    /// (ignored)
    #[arg(short = 'u', default_value_t = false)]
    unbuffered: bool,
    /// Use ^ and M- notation, except for LFD and TAB
    #[arg(short = 'v', long = "show-nonprinting", default_value_t = false)]
    show_nonprinting: bool,
    /// file(s) to read
    #[arg(num_args = 0..)]
    files: Vec<std::path::PathBuf>,
}

impl Cli {
    fn is_plain(&self) -> bool {
        !self.number_nonblank
            && !self.combination_e
            && !self.show_ends
            && !self.number
            && !self.squeeze_blank
            && !self.combination_t
            && !self.show_tabs
            && !self.show_nonprinting
    }
}

#[derive(Copy, Clone)]
enum NumberMode {
    None,
    All,
    NonBlank,
}

fn process_reader(
    reader: &mut dyn BufRead,
    show_tabs: bool,
    show_nonprinting: bool,
    show_ends: bool,
    squeeze_blank: bool,
    mode: NumberMode,
    handle: &mut dyn Write,
    count: &mut u64,
    squeeze_count: &mut usize,
) -> Result<()> {
    let mut buf = Vec::new();

    loop {
        buf.clear();
        let bytes_read = reader
            .read_until(b'\n', &mut buf)
            .with_context(|| "Failed to read input")?;

        if bytes_read == 0 {
            break;
        }

        let has_newline = buf.ends_with(b"\n");
        let line = if has_newline {
            &buf[..buf.len() - 1]
        } else {
            &buf[..]
        };

        if line.is_empty() && squeeze_blank {
            *squeeze_count += 1;
            if *squeeze_count > 1 {
                continue;
            }
        } else {
            *squeeze_count = 0;
        }

        let rendered = render_line(line, show_tabs, show_nonprinting, show_ends);

        match mode {
            NumberMode::None => {
                handle
                    .write_all(&rendered)
                    .with_context(|| "Unable to print contents")?;
            }
            NumberMode::All => {
                write!(handle, "{:<4}{}{:<2}", "", *count, "")
                    .with_context(|| "Unable to print contents")?;
                handle
                    .write_all(&rendered)
                    .with_context(|| "Unable to print contents")?;
                *count += 1;
            }
            NumberMode::NonBlank => {
                if line.is_empty() {
                    if show_ends {
                        handle
                            .write_all(b"$")
                            .with_context(|| "Unable to print contents")?;
                    }
                } else {
                    write!(handle, "{:<4}{}{:<2}", "", *count, "")
                        .with_context(|| "Unable to print contents")?;
                    handle
                        .write_all(&rendered)
                        .with_context(|| "Unable to print contents")?;
                    *count += 1;
                }
            }
        }

        if has_newline {
            handle
                .write_all(b"\n")
                .with_context(|| "Unable to print contents")?;
        }
    }

    Ok(())
}

fn render_line(line: &[u8], show_tabs: bool, show_nonprinting: bool, show_ends: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(line.len() + 1);

    for &b in line {
        if show_tabs && b == b'\t' {
            out.extend_from_slice(b"^I");
            continue;
        }

        if show_nonprinting {
            match b {
                0..=8 | 11..=31 => {
                    out.push(b'^');
                    out.push(b + 64);
                }
                127 => out.extend_from_slice(b"^?"),
                128..=255 => {
                    out.extend_from_slice(b"M-");
                    let low = b - 128;
                    match low {
                        0..=8 | 11..=31 => {
                            out.push(b'^');
                            out.push(low + 64);
                        }
                        127 => out.extend_from_slice(b"^?"),
                        _ => out.push(low),
                    }
                }
                _ => out.push(b),
            }
        } else {
            out.push(b);
        }
    }

    if show_ends {
        out.push(b'$');
    }

    out
}

fn simple_cato(args: Cli) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let files = if args.files.is_empty() {
        vec![std::path::PathBuf::from("-")]
    } else {
        args.files
    };

    for path in files {
        if path.as_os_str() == "-" {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin.lock());
            for line in reader.lines() {
                let line = line.with_context(|| "Failed to read from stdin")?;
                writeln!(handle, "{}", line)
                    .with_context(|| "Unable to print file contents")?;
            }
        } else {
            let file = File::open(&path)
                .with_context(|| format!("Unable to read file `{}`", path.display()))?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line
                    .with_context(|| format!("Unable to read file `{}`", path.display()))?;
                writeln!(handle, "{}", line)
                    .with_context(|| "Unable to print file contents")?;
            }
        }
    }

    Ok(())
}

fn cato(mut args: Cli, mode: NumberMode) -> Result<()> {
    let mut count: u64 = 1;
    let mut squeeze_count: usize = 0;
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    if args.combination_e {
        args.show_nonprinting = true;
        args.show_ends = true;
    }

    if args.combination_t {
        args.show_nonprinting = true;
        args.show_tabs = true;
    }

    let files = if args.files.is_empty() {
        vec![std::path::PathBuf::from("-")]
    } else {
        args.files
    };

    for path in files {
        if path.as_os_str() == "-" {
            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin.lock());
            process_reader(
                &mut reader,
                args.show_tabs,
                args.show_nonprinting,
                args.show_ends,
                args.squeeze_blank,
                mode,
                &mut handle,
                &mut count,
                &mut squeeze_count,
            )
            .with_context(|| "Failed to read stdin")?;
        } else {
            let file = File::open(&path)
                .with_context(|| format!("Unable to read file {}", path.display()))?;
            let mut reader = BufReader::new(file);
            process_reader(
                &mut reader,
                args.show_tabs,
                args.show_nonprinting,
                args.show_ends,
                args.squeeze_blank,
                mode,
                &mut handle,
                &mut count,
                &mut squeeze_count,
            )
            .with_context(|| format!("Unable to read file {}", path.display()))?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.is_plain() {
        return simple_cato(args);
    }

    let mode = if args.number_nonblank {
        NumberMode::NonBlank
    } else if args.number {
        NumberMode::All
    } else {
        NumberMode::None
    };

    cato(args, mode)?;

    Ok(())
}
