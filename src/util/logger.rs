use std::fs::OpenOptions;
use std::io::prelude::*;

use time::{get_time, at};

pub fn log(content: &str) {
    let time = at(get_time());
    let name = format!(
        "log\\log_{}_{}_{}.txt",
        time.tm_year + 1900,
        time.tm_mon + 1,
        time.tm_mday,
    );
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(name)
        .unwrap();
    let content = format!(
        "{} INFO: {}",
        time.rfc3339(),
        content,
    );
    if let Err(e) = writeln!(file, "{}", content) {
        println!("{}", e);
    }
}
