use std::{env::args, fs, path::Path};
use walkdir::WalkDir;
use rayon::prelude::*;
use reflink::reflink_or_copy;

// https://github.com/prevter/slic/blob/10a1f6f76d76d1b549997a76bab20a06e9198499/include/slic.hpp#L350-L353
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLDLINE: &str = "\x1b[1m\x1b[4m";

fn print_help() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!(
        "{ANSI_BOLDLINE}Usage:{ANSI_RESET} fcp [OPTIONS] INPUT OUTPUT\n\n\
        Superfast, multithreaded & cross-platform file copy utility - {VERSION}\n\n\
        {ANSI_BOLDLINE}Arguments:{ANSI_RESET}\n\
        \x20 {ANSI_BOLD}INPUT{ANSI_RESET} Input file or directory to copy\n\
        \x20 {ANSI_BOLD}OUTPUT{ANSI_RESET} Destination path\n\n\
        {ANSI_BOLDLINE}Options:{ANSI_RESET}\n\
        \x20 {ANSI_BOLD}-h, --help{ANSI_RESET} Show this help message"
    );
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 || args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        return;
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    if !input_path.exists() {
        eprintln!("{} does not exist!", input_path.display());
        return;
    }

    if input_path.is_dir() {
        WalkDir::new(input_path)
            .into_iter()
            .filter_map(|result| result.ok())
            .filter(|file| file.path().is_file())
            .par_bridge()
            .for_each(|file| {
                let file_path = file.path();

                if let Ok(relative_path) = file_path.strip_prefix(input_path) {
                    let target_path = output_path.join(relative_path);
                    
                    if let Some(parent_path) = target_path.parent() {
                        if let Err(error) = fs::create_dir_all(parent_path) {
                            eprintln!("Failed to create directory {}: {error}", parent_path.display());
                            return;
                        }
                    }

                    if let Err(error) = reflink_or_copy(file_path, &target_path) {
                        eprintln!("Skipped {}: {error}", file_path.display());
                    }
                }
            });
    } else if let Err(error) = reflink_or_copy(input_path, output_path) {
        eprintln!("Failed to copy {}: {error}", input_path.display());
    }
}