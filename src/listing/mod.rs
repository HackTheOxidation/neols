pub mod formatting;

use crate::listing::formatting::*;
use crate::options::CliOptions;
use colored::*;
use std::fs;
use std::io;

/// Lists the contents of a directory (`cwd`).
///
/// `hidden` determines whether hidden content will be shown.
fn list_default(cwd: &str, hidden: bool) {
    let dirs = fs::read_dir(cwd);

    if let Ok(entries) = dirs {
        for entry in entries.flatten() {
            let name = entry.file_name().into_string();
            if let Ok(name) = name {
                if name.starts_with('.') && hidden {
                    continue;
                }
                match entry.file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
                            print!("{}  ", name.blue().bold());
                        } else if file_type.is_symlink() {
                            print!("{}  ", name.magenta().bold());
                        } else {
                            print!("{}  ", name);
                        }
                    }
                    Err(_) => print!("{}  ", name),
                }
            }
        }
    }

    println!();
}

/// Dispatches calls to an other listing function based on commandline options
///
/// # Panics
///
/// See `validate_options()`
pub fn list_content(cwd: String, options: CliOptions) {
    let options = options.validate_options();
    let hidden = !options.all;

    if options.dirs_only {
        list_dirs_only(cwd);
    } else if options.long_format {
        list_long_format(cwd, hidden);
    } else {
        list_default(cwd.as_str(), hidden);
    }
}

/// List the contents of a directory with ReadOnly Size and Name
///
/// # Panics
///
/// If the supplied directory (`cwd`) does not exist.
/// Or if the file_type of an entry cannot be retrieved.
fn list_long_format(cwd: String, hidden: bool) {
    let dirs = fs::read_dir(cwd.clone());

    match dirs {
        Ok(entries) => {
            println!("ReadOnly Size Name");
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    if let Ok(metadata) = entry.metadata() {
                        if name.starts_with('.') && hidden {
                            continue;
                        }
                        match entry.file_type() {
                            Ok(file_type) => {
                                print_metadata(metadata);

                                if file_type.is_dir() {
                                    println!("{}", name.blue().bold());
                                } else if file_type.is_symlink() {
                                    println!("{}", name.magenta().bold());
                                } else {
                                    println!("{}", name);
                                }
                            }
                            Err(e) => panic!("neols: Error - {}", e),
                        }
                    }
                }
            }
        }
        Err(_) => panic!("neols: Error - Directory does not exist: {}", cwd),
    }
}

/// Lists only the directories in the supplied directory (`cwd`)
fn list_dirs_only(cwd: String) {
    let dir_closure = |entry: Result<fs::DirEntry, io::Error>| {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if let Ok(name) = entry.file_name().into_string() {
                        print!("{}  ", name.blue().bold());
                    }
                }
            }
        }
    };

    fs::read_dir(cwd)
        .into_iter()
        .flatten()
        .for_each(dir_closure);

    println!();
}
