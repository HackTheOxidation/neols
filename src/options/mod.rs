/// Struct for holding all commandline arguments and options
#[derive(Clone)]
pub struct CliOptions {
    pub all: bool,
    pub long_format: bool,
    pub dirs_only: bool,
    pub directory: String,
}

/// Constructs a CliOptions struct from a Vec<String> of arguments
///
/// # Panics
///
/// If a commandline argument is not known. Known options: `-a`, `-d` and `-l`
pub fn build_options_from_args(args: Vec<String>) -> CliOptions {
    let mut options = CliOptions::new();
    let mut dir_set = false;

    for arg in &args {
        if arg.starts_with('-') {
            if arg == "-a" {
                options.all = true;
            } else if arg == "-l" {
                options.long_format = true;
            } else if arg == "-d" {
                options.dirs_only = true;
            } else {
                panic!("neols: Error - Unknown argument: {}", arg);
            }
        } else if !arg.ends_with("neols") {
            if !dir_set {
                options.directory = arg.to_string();
                dir_set = true;
            } else {
                panic!("neols: Error - Unknown argument: {}", arg);
            }
        }
    }

    options
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
        }
    }
}
