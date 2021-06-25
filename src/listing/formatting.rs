use colored::*;
use std::fs;

/// Prints the size (in Bytes) of the file and whether it is ReadOnly
pub fn print_metadata(metadata: fs::Metadata) {
    let mut formatted: String;
    if metadata.permissions().readonly() {
        formatted = "  Yes    ".to_string().red().to_string();
    } else {
        formatted = "  No     ".to_string().green().to_string();
    }

    formatted.push_str(
        format!("{} ", format_bytes(metadata.len()))
            .yellow()
            .to_string()
            .as_str(),
    );

    while formatted.chars().count() < 31 {
        formatted.push(' ');
    }

    print!("{} ", formatted);
}

/// Converts a number of bytes into a String with appropriate units
fn format_bytes(bytes: u64) -> String {
    let mut bytes: f64 = bytes as f64;

    if bytes >= 1_000_000_000.0 {
        bytes /= 1_000_000_000.0;
        let mut bytes = bytes.round().to_string();
        bytes.push('G');

        return bytes;
    }

    if bytes >= 1_000_000.0 {
        bytes /= 1_000_000.0;
        let mut bytes = bytes.round().to_string();
        bytes.push('M');

        return bytes;
    }

    if bytes >= 1_000.0 {
        bytes /= 1_000.0;
        let mut bytes = bytes.round().to_string();
        bytes.push('K');

        return bytes;
    }

    bytes.round().to_string()
}
