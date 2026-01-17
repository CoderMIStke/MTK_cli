use std::env;
use std::fs;
use std::path::PathBuf;

struct Args {
    pwd: PathBuf,
    input: PathBuf,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let pwd = env::current_dir()
            .map_err(|e| format!("failed to get current working directory: {}", e))?;
        let mut input: Option<String> = None;
        let mut iter = env::args().skip(1);
        while let Some(arg) = iter.next() {
            if arg == "-i" {
                input = iter.next();
                break;
            }
        }
        let input = input.ok_or("missing -i <file> argument")?;

        Ok(Self { pwd, input: input.into() })
    }
}

fn main() {
    let args = Args::parse().unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    let path =
        if args.input.is_absolute() { args.input.clone() } else { args.pwd.join(&args.input) };

    let content = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("failed to read {}: {e}", path.display());
        std::process::exit(1);
    });

    println!("{content}");
}
