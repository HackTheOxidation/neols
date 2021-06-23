pub struct CliOptions {
    pub all: bool,
    pub long_format: bool,
}

pub fn build_options_from_args(args: Vec<String>) -> CliOptions {
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

pub fn validate_options(options: CliOptions) -> CliOptions {
    if options.long_format && options.all {
        panic!("neols: Error - Incompatible arguments: -a and -l");
    }

    options
}
