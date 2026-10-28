use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let path: &str = &args[1];

            match File::open(path) {
                Ok(mut file) => {
                    if let Err(error) = print_file_info(&file, path) {
                        eprintln!("rat: {path}: {error}");
                        return ExitCode::FAILURE;
                    }

                    if let Err(error) = print_file_content(&mut file) {
                        eprintln!("rat: {path}: {error}");
                        return ExitCode::FAILURE;
                    }
                }
                Err(error) => {
                    eprintln!("rat: {path}: {error}");
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

fn print_file_info(file: &File, path: &str) -> io::Result<()> {
    let metadata = file.metadata()?;

    let file_type = if metadata.is_file() {
        "Regular file"
    } else if metadata.is_dir() {
        "Directory"
    } else if metadata.is_symlink() {
        "Symbolic link"
    } else {
        "Other"
    };

    writeln!(io::stdout(), "File: {path}")?;
    writeln!(io::stdout(), "Type: {file_type}")?;
    writeln!(io::stdout(), "Size: {} bytes", metadata.len())?;
    writeln!(io::stdout(), "------------------------")?;

    Ok(())
}

fn print_file_content(file: &mut File) -> io::Result<()> {
    io::copy(file, &mut io::stdout())?;

    Ok(())
}
