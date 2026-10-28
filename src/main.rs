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
                        if let Err(error) = print_directory_tree(path, 0, -1) {
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

fn print_directory_tree(path: &Path, depth: usize, max_depth: i32) -> io::Result<()> {
    let mut entries = fs::read_dir(path)?.collect::<io::Result<Vec<_>>>()?;

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let entry_path = entry.path();

        for _ in 0..depth {
            print!("    ");
        }

        println!("{}", entry.file_name().to_string_lossy());

        if entry.file_type()?.is_dir() && (max_depth < 0 || depth < max_depth as usize) {
            print_directory_tree(&entry_path, depth + 1, max_depth)?;
        }
    }

    Ok(())
}
