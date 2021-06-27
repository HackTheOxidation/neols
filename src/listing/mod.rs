pub mod formatting;

use crate::listing::formatting::*;
use crate::options::CliOptions;
use colored::*;
use std::fs;

/// Lists the contents of a directory (`cwd`).
///
/// `hidden` determines whether hidden content will be shown.
fn list_default(cwd: String, hidden: bool) {
    sort_content(get_dirs(cwd, hidden))
        .iter()
        .for_each(|entry| {
            let name = entry.file_name().into_string().unwrap();
            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() {
                print!("{}  ", name.blue().bold());
            } else if file_type.is_symlink() {
                print!("{}  ", name.magenta().bold());
            } else {
                print!("{}  ", name);
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
        list_default(cwd, hidden);
    }
}

/// Extracts and filters content in a directory (cwd)
fn get_dirs(cwd: String, hidden: bool) -> Vec<fs::DirEntry> {
    fs::read_dir(cwd)
        .into_iter()
        .flatten()
        .map(|entry| entry.unwrap())
        .filter(|entry| {
            if hidden {
                !(entry.file_name().into_string().unwrap().starts_with('.'))
            } else {
                true
            }
        })
        .collect::<Vec<fs::DirEntry>>()
}

/// Sorts contents in a directory lexicographically
fn sort_content(mut dirs: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
    dirs.sort_by(|a, b| {
        a.file_name()
            .into_string()
            .unwrap()
            .cmp(&b.file_name().into_string().unwrap())
    });
    dirs
}

/// List the contents of a directory with ReadOnly Size and Name
fn list_long_format(cwd: String, hidden: bool) {
    let dirs = sort_content(get_dirs(cwd, hidden));

    dirs.iter().for_each(|entry| {
        let name = entry.file_name().into_string().unwrap();
        let file_type = entry.file_type().unwrap();
        print_metadata(entry.metadata().unwrap());
        if file_type.is_dir() {
            println!("{}  ", name.blue().bold());
        } else if file_type.is_symlink() {
            println!("{}  ", name.magenta().bold());
        } else {
            println!("{}  ", name);
        }
    });
}

/// Lists only the directories in the supplied directory (`cwd`)
fn list_dirs_only(cwd: String) {
    sort_content(get_dirs(cwd, true)).iter().for_each(|entry| {
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            print!(
                "{}  ",
                entry.file_name().into_string().unwrap().blue().bold()
            );
        }
    });

    println!();
}
