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
            print_rat();
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

fn print_rat() {
    const ART: &[&str] = &[
        "     pppp",
        "    pPPppPp",
        "   pPpmmmpPp                             OO",
        "  pPp     pP                      OdllO dmmm",
        " pPp       P                     OmppPlOddmPp",
        " Pp       pp           OOOOO     dpPPPPlOOmPP",
        "pPp     pPp        ffOmmmmmmOdO  mPPPPPpdOdpP",
        "pPp               OOmmmmmmmmmmdO mPPPPPpmOOpP                           f",
        "pPp             OddmmmmmllmmmmmddOPPPPppmOmdO                          pHp",
        "pPp           OOmmmmmmmmmmmmmmmmdOmPPPPOmdmmmdO                      pHHoHp",
        " PPp         OddmmmmmmmmmmmmmlmmmdOdpPppmmllmldO                    yHHHHHHp",
        " pPP        OdddmmmmmmmmmmmmmmmmmmddOOmmmmmmmllm                  pHHyyyoHHY",
        "  pPp       OOmdmmmlmmmmmmmmmmmmmmmmmmmmllmdOOmlm     l         pHHyHyoooHHHp",
        "  OpPp     fOdmmmmlllmmmmmmmmmmmmmmmmmmlllmOpOmmlOll l        fyHHHHHYooYHHHYf",
        "    PPPOO  OddmmmmllllmmmmmmmmmmmmmmmmmmmlmmOpmmmmdO lll      YHoHHHHHHHHHHHyp",
        "     pPPPPpOddmmmmmlllmddmmmmmmmmmlmmmmmmmmmmmmmmpPp        pHHHoHHHHHHHYoYYop",
        "      pppppOdddmmmmmmlmmddmmmmmmmmmmmmmmfffffflfffpd  l    YHHHHHHYYYYYYYYYYop",
        "        pppOOdmmmmmmmmmdddmmmmmmmmmmmmmmfffflffldO       pHHYYYYYYYYYYyyYYYYop",
        "           OOddmmmmmmmmddmmmmmmdmmmmlllfllllldf l        pYYYYYYYYYYYooooYYYop",
        "            OOddddmmmmdmmmmffmdmmmlllffllf      l        pYYYYyyYYYoYYooHYYYop",
        "            OOddmmmdddmlfffffflmmmlfmlfmd    l            yYYyooyYYYYYHYYYYYop",
        "             pmmddOdOlffflfffllldmlfflOO                  pYYyoooYYYYYYYYoYYop",
        "             pPpOOOppOlOOOmmOlOOOdmp  OPpp               pYYYHooYYYYoyYYYYYYop",
        "             pPPp   pPPm          pPPp pPpp              pYYYYYYYYYYyYYYYYYYop",
        "              PPPp  ppp           pPPPp ppp              pYYYYYYYYYYHYYYYYYYop",
        "               ppp                 pppp",
    ];

    fn color(pixel: Option<&u8>) -> Option<(u8, u8, u8)> {
        Some(match pixel.copied()? {
            // Mouse outline and layered blue-gray fur.
            b'O' => (37, 45, 56),
            b'd' => (72, 88, 110),
            b'm' => (112, 133, 160),
            b'l' => (165, 184, 204),
            b'f' => (231, 232, 226),

            // Pink tail, ears, paws, and nose.
            b'p' => (190, 86, 109),
            b'P' => (246, 151, 162),

            // Cheese, highlights, and holes.
            b'y' => (248, 166, 20),
            b'Y' => (255, 205, 55),
            b'H' => (255, 233, 126),
            b'o' => (184, 83, 8),

            // Whiskers.
            b'w' => (191, 183, 170),

            _ => return None,
        })
    }

    let width = ART.iter().map(|row| row.len()).max().unwrap_or(0);

    for rows in ART.chunks(2) {
        let top = rows[0].as_bytes();
        let bottom = rows.get(1).map_or(&[][..], |row| row.as_bytes());

        for column in 0..width {
            let upper = color(top.get(column));
            let lower = color(bottom.get(column));

            match (upper, lower) {
                (Some((r1, g1, b1)), Some((r2, g2, b2))) => {
                    print!("\x1b[38;2;{r1};{g1};{b1}m\x1b[48;2;{r2};{g2};{b2}m▀");
                }
                (Some((r, g, b)), None) => {
                    print!("\x1b[38;2;{r};{g};{b}m▀");
                }
                (None, Some((r, g, b))) => {
                    print!("\x1b[38;2;{r};{g};{b}m▄");
                }
                (None, None) => print!(" "),
            }

            print!("\x1b[0m");
        }

        println!();
    }

    println!();
    println!("  \x1b[38;2;255;205;55mrat\x1b[0m — a little Rusty mouse chasing a C cat.");
    println!("  Usage: \x1b[38;2;165;184;204mrat <path>\x1b[0m");
    println!("  Files: print contents  |  Directories: expand a tree");
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
