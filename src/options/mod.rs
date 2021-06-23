#[derive(Copy, Clone)]
pub struct CliOptions {
    pub all: bool,
    pub long_format: bool,
}

pub fn build_options_from_args(args: Vec<String>) -> CliOptions {
    let mut options = CliOptions::new();

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

impl CliOptions {
    pub fn validate_options(&self) -> self::CliOptions {
        if self.long_format && self.all {
            panic!("neols: Error - Incompatible arguments: -a and -l");
        }

        *self
    }

    pub fn new() -> CliOptions {
        CliOptions {
            all: false,
            long_format: false,
        }
    }
}
