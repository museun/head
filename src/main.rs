use std::io::prelude::*;

fn die<F: Fn() -> D, D: std::fmt::Display>(f: F) -> ! {
    eprintln!("{}", f());
    std::process::exit(1);
}

fn print_help() -> ! {
    static HELP: &str = r#"usage:
    head [flags] [FILE]..

flags:
    -c, --bytes NUM  number of bytes to read
    -n, --lines NUM  number of lines to read
    -q, --quiet      if multiple files are provided disable the header
    -h, --help       print this message

    if no flags were provided, 10 lines will be read    
    if 'FILE' is - then stdin will be read
"#;
    println!("{}", HELP);
    std::process::exit(0);
}

fn get_val(
    args: &mut pico_args::Arguments,
    keys: impl Into<pico_args::Keys> + std::fmt::Debug + Copy,
) -> Option<u64> {
    args.opt_value_from_str(keys).unwrap_or_else(|_| {
        die(|| {
            format!("argument for {:?} must be parsable as an u64", keys) //
        })
    })
}

#[derive(Copy, Clone)]
enum Filter {
    Bytes(u64),
    Lines(u64),
}

fn read(read: &mut dyn BufRead, filter: Filter) -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    match filter {
        Filter::Bytes(n) => std::io::copy(&mut read.take(n), &mut stdout).map(|_| ()),
        Filter::Lines(n) => read
            .lines()
            .flatten()
            .take(n as _)
            .map(|line| writeln!(&mut stdout, "{}", line))
            .collect(),
    }
}

fn main() {
    let mut args = pico_args::Arguments::from_env();
    if args.contains(["-h", "--help"]) {
        print_help()
    }

    let bytes = get_val(&mut args, ["-c", "--bytes"]);
    let lines = get_val(&mut args, ["-n", "--lines"]);
    let mut quiet = args.contains(["-q", "--quiet"]);

    let filter = match (bytes, lines) {
        (Some(..), Some(..)) => die(|| "[-c,--bytes] is exclusive with [-n,--lines]"),
        (Some(bytes), None) => Filter::Bytes(bytes),
        (None, Some(lines)) => Filter::Lines(lines),
        (..) => Filter::Lines(10),
    };

    let files = args.free().unwrap();
    if files.is_empty() {
        die(|| "provide atleast one [FILE].. or '-' for stdin");
    }

    quiet |= files.len() == 1;

    for path in files {
        if !quiet {
            println!("==> {} <==", path);
        }

        let (mut stdin, mut file);
        let input: &mut dyn Read = if path == "-" {
            stdin = std::io::stdin();
            &mut stdin
        } else {
            file = match std::fs::File::open(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("==> WARN: cannot open '{}': {}", path, err);
                    continue;
                }
            };
            &mut file
        };

        if let Err(err) = read(&mut std::io::BufReader::new(input), filter) {
            die(|| format!("cannot read '{}': {}", path, err));
        }

        if !quiet {
            println!();
        }
    }
}
