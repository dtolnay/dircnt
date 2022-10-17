use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    if let Err(error) = try_main() {
        let _ = writeln!(io::stderr(), "dircnt: {}", error);
        process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    let mut args_os = env::args_os().skip(1);
    let dir = args_os.next();

    let read_dir = match dir {
        Some(flag) if flag == "--version" => {
            let _ = writeln!(io::stdout(), "dircnt {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        Some(dir) => fs::read_dir(dir),
        None => fs::read_dir("."),
    }?;

    let mut count = 0usize;
    for entry in read_dir {
        entry?;
        count += 1;
    }
    let _ = writeln!(io::stdout(), "{}", count);
    Ok(())
}
