pub mod formatting;

use crate::listing::formatting::*;
use crate::options::CliOptions;
use colored::*;
use std::fs;

/// Lists the contents of a directory (`cwd`).
///
/// `hidden` determines whether hidden content will be shown.
fn list_default(cwd: &str, hidden: bool) {
    fs::read_dir(cwd).into_iter().flatten().for_each(|entry| {
        if let Ok(entry) = entry {
            let name = entry.file_name().into_string().unwrap();
            if !(name.starts_with('.') && hidden) {
                let file_type = entry.file_type().unwrap();
                if file_type.is_dir() {
                    print!("{}  ", name.blue().bold());
                } else if file_type.is_symlink() {
                    print!("{}  ", name.magenta().bold());
                } else {
                    print!("{}  ", name);
                }
            }
        }
    });

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
fn list_long_format(cwd: String, hidden: bool) {
    fs::read_dir(cwd).into_iter().flatten().for_each(|entry| {
        if let Ok(entry) = entry {
            let name = entry.file_name().into_string().unwrap();
            let metadata = entry.metadata().unwrap();
            if !(name.starts_with('.') && hidden) {
                let file_type = entry.file_type().unwrap();
                print_metadata(metadata);
                if file_type.is_dir() {
                    println!("{}  ", name.blue().bold());
                } else if file_type.is_symlink() {
                    println!("{}  ", name.magenta().bold());
                } else {
                    println!("{}  ", name);
                }
            }
        }
    });
}

/// Lists only the directories in the supplied directory (`cwd`)
fn list_dirs_only(cwd: String) {
    fs::read_dir(cwd).into_iter().flatten().for_each(|entry| {
        if let Ok(entry) = entry {
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                print!(
                    "{}  ",
                    entry.file_name().into_string().unwrap().blue().bold()
                );
            }
        }
    });

    println!();
}
