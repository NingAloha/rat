use std::env;
use std::fs::File;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rat <file>");
        return ExitCode::FAILURE;
    }

    let path: &str = &args[1];

    match File::open(path) {
        Ok(mut file) => {
            let mut stdout = io::stdout();

            match io::copy(&mut file, &mut stdout) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("rat: {path}: {error}");
                    return ExitCode::FAILURE;
                }
            }
        }
        Err(error) => {
            eprintln!("rat: {path}: {error}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
