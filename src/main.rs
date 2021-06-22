use std::fs;
use colored::*;
use std::env;

struct CliOptions {
   all: bool,
   long_format: bool,
}

fn list_default(cwd: &str, hidden: bool) {
    let dirs = fs::read_dir(cwd);

    if let Ok(entries) = dirs {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().into_string();
                if let Ok(name) = name {
                    if name.starts_with(".") && hidden {
                        continue;
                    }
                    match entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_dir() {
                                print!("{}  ", name.blue());
                            } else if file_type.is_symlink() {
                                print!("{}  ", name.magenta());
                            } else {
                                print!("{}  ", name);
                            }
                        },
                        Err(_) => print!("{}  ", name),
                    }
                }
            }
        }
    }
    
    println!();
}

fn list_content(cwd: String, options: CliOptions) {
    let options = validate_options(options);

    if options.all {
        list_default(cwd.as_str(), false);
    } else if options.long_format {
        list_long_format(cwd);
    }
}

fn list_long_format(cwd: String) {
    let dirs = fs::read_dir(cwd.clone());
    
    match dirs {
        Ok(entries) => {
            println!("ReadOnly Size Name");
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(name) = entry.file_name().into_string() {
                        if let Ok(metadata) = entry.metadata() {
                            match entry.file_type() {
                                Ok(file_type) => {
                                    print_metadata(metadata);

                                    if file_type.is_dir() {
                                        println!("{}", name.blue());
                                    } else if file_type.is_symlink() {
                                        println!("{}", name.magenta());
                                    } else {
                                        println!("{}", name);
                                    }
                                },
                                Err(e) => panic!("neols: Error - {}", e),
                            }
                        }
                    }
                }
            }
        },
        Err(_) => panic!("neols: Error - Directory does not exist: {}", cwd),
    }
}

fn print_metadata(metadata: fs::Metadata) {
    if metadata.permissions().readonly() {
        print!("  Yes    ");
    } else {
        print!("  No     ");
    }

    print!("{} ", format_bytes(metadata.len()));

}

fn format_bytes(bytes: u64) -> String {
    
    let mut bytes: f64 = bytes as f64;

    if bytes >= 1_000_000_000.0 {
        bytes = bytes / 1_000_000_000.0;
        bytes.to_string().push_str("G")
    }

    if bytes >= 1_000_000.0 {
        bytes = bytes / 1_000_000.0;
        bytes.to_string().push_str("M")
    }

    if bytes >= 1_000.0 {
        bytes = bytes / 1_000.0;
        bytes.to_string().push_str("K")
    }

    bytes.to_string()
}

fn build_options_from_args(args: Vec<String>) -> CliOptions {
    let mut options = CliOptions {
        all: false,
        long_format: false,
    };

    for arg in &args {
        if arg == "-a" {
            options.all = true;
        } else if arg == "-l" {
            options.long_format = true;
        } else {
            panic!("neols: Error - Unknown argument: {}", arg);
        }
    }

    options
}

fn validate_options(options: CliOptions) -> CliOptions {
    if options.long_format && options.all {
        panic!("neols: Error - Incompatible arguments: -a and -l");
    }

    options
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        list_default(".", true);
    } else {
        let options = build_options_from_args(
            args.into_iter().filter(|s| s.starts_with("-")).collect());

        list_content(String::from("."), options);
    }
}
