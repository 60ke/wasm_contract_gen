// #[macro_use]
// extern crate log;
use env_logger::{fmt::Color, Builder, Env};
use std::io::Write;

pub fn init_logger() {
    let env = Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_bg(Color::Yellow).set_bold(true);

            let timestamp = buf.timestamp();

            writeln!(
                buf,
                "{}:{} --{}:{} {}",
                timestamp,
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                style.value(record.args())
            )
        })
        .init();
}