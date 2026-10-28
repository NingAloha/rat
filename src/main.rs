mod rathole;

use std::env;
use std::fs;
use std::fs::{File, FileType};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const DISLIKES: &str = include_str!("../.ratdontlike");

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            rathole::rat::comeout();
        }
        2 => {
            let path = Path::new(&args[1]);

            if let Err(error) = run(path) {
                eprintln!("rat: {}: {error}", path.display());
                return ExitCode::FAILURE;
            }
        }
        _ => {
            eprintln!("Usage: rat <file>");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

fn run(path: &Path) -> io::Result<()> {
    let file_type = print_path_info(path)?;

    if file_type.is_file() {
        let mut file = File::open(path)?;
        print_file_content(&mut file)?;
    } else if file_type.is_dir() {
        print_directory_tree(path, 0, -1)?;
    }

    Ok(())
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

    {
        let mut stdout = io::stdout().lock();

        writeln!(stdout, "Path: {}", path.display())?;
        writeln!(stdout, "Type: {type_name}")?;
        writeln!(stdout, "Size: {} bytes", metadata.len())?;
        writeln!(stdout, "------------------------")?;
    }

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

        let name = entry.file_name();
        let name = name.to_string_lossy();

        if DISLIKES
            .lines()
            .map(str::trim)
            .filter(|rule| !rule.is_empty() && !rule.starts_with('#'))
            .any(|rule| rule == name)
        {
            continue;
        }

        {
            let mut stdout = io::stdout().lock();

            for _ in 0..depth {
                write!(stdout, "    ")?;
            }

            writeln!(stdout, "{name}")?;
        }

        if entry.file_type()?.is_dir() && (max_depth < 0 || depth < max_depth as usize) {
            print_directory_tree(&entry_path, depth + 1, max_depth)?;
        }
    }

    Ok(())
}
