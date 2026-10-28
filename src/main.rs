use std::env;
use std::fs;
use std::fs::{File, FileType};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let path = Path::new(&args[1]);

            match print_path_info(path) {
                Ok(file_type) => {
                    if file_type.is_file() {
                        match File::open(path) {
                            Ok(mut file) => {
                                if let Err(error) = print_file_content(&mut file) {
                                    eprintln!("rat: {}: {error}", path.display());
                                    return ExitCode::FAILURE;
                                }
                            }
                            Err(error) => {
                                eprintln!("rat: {}: {error}", &args[1]);
                                return ExitCode::FAILURE;
                            }
                        }
                    } else if file_type.is_dir() {
                        if let Err(error) = print_directory_tree(path) {
                            eprintln!("rat: {}: {error}", path.display());
                            return ExitCode::FAILURE;
                        }
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

fn print_path_info(path: &Path) -> io::Result<FileType> {
    let metadata = fs::metadata(path)?;
    let file_type = metadata.file_type();

    let type_name = if file_type.is_file() {
        "Regular file"
    } else if file_type.is_dir() {
        "Directory"
    } else {
        "Other"
    };

    let mut stdout = io::stdout().lock();

    writeln!(stdout, "Path: {}", path.display())?;
    writeln!(stdout, "Type: {type_name}")?;
    writeln!(stdout, "Size: {} bytes", metadata.len())?;
    writeln!(stdout, "------------------------")?;

    Ok(file_type)
}

fn print_file_content(file: &mut File) -> io::Result<()> {
    io::copy(file, &mut io::stdout())?;
    Ok(())
}

fn print_directory_tree(path: &Path) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}
