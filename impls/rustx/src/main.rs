pub mod util;

use log::info;
use util::environment as env;
use util::logging as lg;

fn initialize() {
    println!("Starting {} ...", env::get_value("CARGO_PKG_NAME"));
    env::initialize();
    lg::initialize();
    println!(
        "{} {} has been successfully started.",
        env::get_value("CARGO_PKG_NAME"),
        env::get_value("CARGO_PKG_VERSION")
    );

    info!(
        "{} {} has been successfully started.",
        env::get_value("CARGO_PKG_NAME"),
        env::get_value("CARGO_PKG_VERSION")
    );
}

fn main() {
    initialize();
}
