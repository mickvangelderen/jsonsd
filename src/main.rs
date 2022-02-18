use std::path::Path;

use clap::{app_from_crate, arg};

fn main() {
    let matches = app_from_crate!()
        .arg(
            arg!([path] "JSON file path to analyze")
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let path = Path::new(matches.value_of_os("path").unwrap());

    jsonsd::analyze(path);
}
