use chrono::Local;
use env_logger;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn initialize() {
    // env_logger::init();
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                // "{} [{}] |{:?}| {}:{} - {}",
                "{} [{}] {}:{} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f%z"),
                record.level(),
                // std::thread::current().name(),
                record.module_path().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
