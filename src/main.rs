mod listing;
mod options;

use crate::listing::list_content;
use crate::options::build_options_from_args;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = build_options_from_args(args);

    list_content(options.directory.clone(), options);
}
