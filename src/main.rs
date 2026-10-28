use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let path= Path::new(&args[1]);

            match File::open(path) {
                Ok(mut file) => {
                    if let Err(error) = print_file_info(path) {
                        eprintln!("rat: {}: {error}", &args[1]);
                        return ExitCode::FAILURE;
                    }

                    if let Err(error) = print_file_content(&mut file) {
                        eprintln!("rat: {}: {error}", &args[1]);
                        return ExitCode::FAILURE;
                    }
                }
                Err(error) => {
                    eprintln!("rat: {}: {error}", &args[1]);
                    return ExitCode::FAILURE;
                }
            }
        }
        _ => {
            eprintln!("Usage: rat <file>");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

fn print_file_info(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;

    let file_type = if metadata.is_file() {
        "Regular file"
    } else if metadata.is_dir() {
        "Directory"
    } else {
        "Other"
    };

    let mut stdout = io::stdout().lock();

    writeln!(stdout, "Path: {}", path.display())?;
    writeln!(stdout, "Type: {file_type}")?;
    writeln!(stdout, "Size: {} bytes", metadata.len())?;
    writeln!(stdout, "------------------------")?;

    Ok(())
}

fn print_file_content(file: &mut File) -> io::Result<()> {
    io::copy(file, &mut io::stdout())?;

    Ok(())
}
