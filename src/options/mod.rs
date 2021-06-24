#[derive(Clone)]
pub struct CliOptions {
    pub all: bool,
    pub long_format: bool,
    pub dirs_only: bool,
    pub directory: String,
}

pub fn build_options_from_args(args: Vec<String>) -> CliOptions {
    let mut options = CliOptions::new();

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
        } else if !arg.ends_with("neols") && options.directory == *"." {
            options.directory = arg.to_string();
        }
    }

    options
}

impl CliOptions {
    pub fn validate_options(&self) -> &self::CliOptions {
        if self.long_format && self.all && self.dirs_only {
            panic!("neols: Error - Incompatible arguments: -a, -d and -l");
        }

        self
    }

    pub fn new() -> CliOptions {
        CliOptions {
            all: false,
            long_format: false,
            dirs_only: false,
            directory: ".".to_string(),
        }
    }
}
