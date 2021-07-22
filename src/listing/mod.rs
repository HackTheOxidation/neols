pub mod formatting;

use crate::listing::formatting::*;
use crate::options::CliOptions;
use colored::*;
use std::fs;
use std::rc::Rc;

/// Lists the contents of a directory (`cwd`).
///
/// `hidden` determines whether hidden content will be shown.
fn list_default(cwd: String, options: &CliOptions) {
    sort_content(get_dirs(cwd, !options.all), options)
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

    if options.dirs_only {
        list_dirs_only(cwd, options);
    } else if options.long_format {
        list_long_format(cwd, options);
    } else {
        list_default(cwd, options);
    }
}

/// Extracts and filters content in a directory (cwd)
fn get_dirs(cwd: String, hidden: bool) -> Vec<Rc<fs::DirEntry>> {
    fs::read_dir(cwd)
        .into_iter()
        .flatten()
        .map(|entry| Rc::new(entry.unwrap()))
        .filter(|entry| {
            if hidden {
                !(entry.file_name().into_string().unwrap().starts_with('.'))
            } else {
                true
            }
        })
        .collect::<Vec<Rc<fs::DirEntry>>>()
}

/// Group the entries by file type (directories and non-directories)
fn group_by(mut dirs: Vec<Rc<fs::DirEntry>>, options: &CliOptions) -> Vec<Rc<fs::DirEntry>> {
    let mut folders: Vec<Rc<fs::DirEntry>> = Vec::new();
    let mut files: Vec<Rc<fs::DirEntry>> = Vec::new();

    let sorting_closure = |a: &fs::DirEntry, b: &fs::DirEntry| {
        a.file_name()
            .into_string()
            .unwrap()
            .cmp(&b.file_name().into_string().unwrap())
    };

    dirs.iter().for_each(|entry| {
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            folders.push(Rc::clone(entry));
        } else {
            files.push(Rc::clone(entry));
        }
    });

    folders.sort_by(|dir1, dir2| {
        if options.reverse_sorted {
            sorting_closure(dir2, dir1)
        } else {
            sorting_closure(dir1, dir2)
        }
    });

    files.sort_by(|file1, file2| {
        if options.reverse_sorted {
            sorting_closure(file2, file1)
        } else {
            sorting_closure(file1, file2)
        }
    });

    dirs.clear();
    if options.reverse_sorted {
        files.iter().for_each(|entry| dirs.push(Rc::clone(entry)));
        folders.iter().for_each(|entry| dirs.push(Rc::clone(entry)));
    } else {
        folders.iter().for_each(|entry| dirs.push(Rc::clone(entry)));
        files.iter().for_each(|entry| dirs.push(Rc::clone(entry)));
    }

    dirs
}

/// Sorts contents in a directory lexicographically
fn sort_content(mut dirs: Vec<Rc<fs::DirEntry>>, options: &CliOptions) -> Vec<Rc<fs::DirEntry>> {
    if options.group_by {
        group_by(dirs, options)
    } else {
        let sorting_closure = |a: &fs::DirEntry, b: &fs::DirEntry| {
            a.file_name()
                .into_string()
                .unwrap()
                .cmp(&b.file_name().into_string().unwrap())
        };

        dirs.sort_by(|dir1, dir2| {
            if options.reverse_sorted {
                sorting_closure(dir2, dir1)
            } else {
                sorting_closure(dir1, dir2)
            }
        });

        dirs
    }
}

/// List the contents of a directory with ReadOnly Size and Name
fn list_long_format(cwd: String, options: &CliOptions) {
    let dirs = sort_content(get_dirs(cwd, !options.all), options);

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
fn list_dirs_only(cwd: String, options: &CliOptions) {
    sort_content(get_dirs(cwd, true), options)
        .iter()
        .for_each(|entry| {
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
