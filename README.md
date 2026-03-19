# cato

A lightweight file catenation tool written in Rust.

## Usage

```bash
Command-line Interface options

Usage: cato [OPTIONS] [FILES]...

Arguments:
  [FILES]...  file(s) to read

Options:
  -n, --number   Number all output lines
  -h, --help     Print help
  -V, --version  Print version
```

## Developer Notes

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Cross-platform Support

| Platform    | Arch        | Toolchain |
| ----------- | ----------- | ------------ |
| Linux       | x86_64      | x86_64-unknown-linux-gnu |
| Linux       | i686        | i686-unknown-linux-gnu | 
| Windows     | x86_64      | x86_64-pc-windows-msvc |             


