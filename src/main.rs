use std::process;

use opal_downloader::error::Error;
use opal_downloader::run;

fn handle_error(err: Error) {
    eprintln!("Error: {err}");
    process::exit(err.exit_code());
}

fn main() {
    let r = run();

    match r {
        Ok(()) => process::exit(0),
        Err(err) => handle_error(err)
    }
}
