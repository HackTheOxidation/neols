/// Struct for holding all commandline arguments and options
#[derive(Clone, Debug)]
pub struct CliOptions {
    pub all: bool,
    pub long_format: bool,
    pub dirs_only: bool,
    pub directory: String,
    pub reverse_sorted: bool,
    pub group_by: bool,
}

/// Constructs a CliOptions struct from a Vec<String> of arguments
///
/// # Panics
///
/// If a commandline argument is not known. Known options: `-a`, `-d` and `-l`
pub fn build_options_from_args(args: Vec<String>) -> CliOptions {
    let mut options = CliOptions::new();
    let mut dir_set = false;

    args.iter().for_each(|arg| {
        if arg.starts_with('-') {
            let mut arg = arg.clone();
            while arg.starts_with('-') {
                arg.remove(0);
            }

            for ch in arg.chars() {
                if ch == 'a' {
                    arg_already_set(options.all, ch);
                    options.all = true;
                } else if ch == 'l' {
                    arg_already_set(options.long_format, ch);
                    options.long_format = true;
                } else if ch == 'd' {
                    arg_already_set(options.dirs_only, ch);
                    options.dirs_only = true;
                } else if ch == 'r' {
                    arg_already_set(options.reverse_sorted, ch);
                    options.reverse_sorted = true;
                } else if ch == 'g' {
                    arg_already_set(options.group_by, ch);
                    options.group_by = true;
                } else {
                    panic!("neols: Error - Unknown argument: {}", ch);
                }
            }
        } else if !arg.ends_with("neols") {
            if !dir_set {
                options.directory = arg.to_string();
                dir_set = true;
            } else {
                panic!("neols: Error - Unknown argument: {}", arg);
            }
        }
    });

    options
}

fn arg_already_set(condition: bool, ch: char) {
    if condition {
        panic!("neols: Error - Argument already set: {}", ch);
    }
}

impl CliOptions {
    /// Checks if commandline arguments and options are compatible
    ///
    /// # Panics
    ///
    ///
    pub fn validate_options(&self) -> &self::CliOptions {
        if (self.long_format || self.all) && self.dirs_only {
            panic!("neols: Error - Incompatible arguments: -a, -d and -l");
        }

        self
    }

    /// Creates a CliOptions struct with default values
    pub fn new() -> CliOptions {
        CliOptions {
            all: false,
            long_format: false,
            dirs_only: false,
            directory: ".".to_string(),
            reverse_sorted: false,
            group_by: false,
        }
    }
}
