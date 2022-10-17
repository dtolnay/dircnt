//! [![github]](https://github.com/dtolnay/dircnt)&ensp;[![crates-io]](https://crates.io/crates/dircnt)&ensp;[![docs-rs]](https://docs.rs/dircnt)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

#![allow(clippy::let_underscore_drop, clippy::uninlined_format_args)]

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

const USAGE: &str = "usage: dircnt [DIR]\n";

fn main() {
    if let Err(error) = try_main() {
        let _ = writeln!(io::stderr(), "dircnt: {}", error);
        process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    let mut args_os = env::args_os().skip(1);
    let dir = args_os.next();
    if args_os.next().is_some() {
        let _ = write!(io::stderr(), "{}", USAGE);
        process::exit(1);
    }

    let read_dir = match dir {
        Some(flag) if flag == "--help" => {
            let _ = write!(io::stdout(), "{}", USAGE);
            process::exit(0);
        }
        Some(flag) if flag == "--version" => {
            let _ = writeln!(io::stdout(), "dircnt {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        Some(dir) => fs::read_dir(dir),
        None => fs::read_dir("."),
    }?;

    let mut count = 0usize;
    for entry in read_dir {
        if let Err(error) = entry {
            let _ = writeln!(io::stderr(), "dircnt: {}", error);
        } else {
            count += 1;
        }
    }
    let _ = writeln!(io::stdout(), "{}", count);
    Ok(())
}
