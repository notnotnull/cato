# cato

A lightweight file catenation tool written in Rust.

## Usage

```bash
A lightweight file catenation tool.

Usage: cato [OPTIONS] [FILES]...

Arguments:
  [FILES]...  file(s) to read

Options:
  -b, --number-nonblank  Number nonempty output lines, overrides -n
  -n, --number           Number all output lines
  -h, --help             Print help
  -V, --version          Print version
```

## Install

### Windows 64-bit

```ps
choco install cato
```

### Cargo

```
cargo install cato
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


