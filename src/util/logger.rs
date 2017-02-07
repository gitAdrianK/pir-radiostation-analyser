use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;
use time::{get_time, at};

/// Log a message in a textfile log folder log
/// Also includes the date
pub fn log(content: &str) {
    log_at("log", "log", content);
}

/// Logs a message at a directory with given name
/// Also includes the date
pub fn log_at(dir: &str, name: &str, content: &str) {
    // Create all directorys
    let _ = fs::create_dir_all(dir);
    // Get system time
    let time = at(get_time());
    let name = format!(
        "{}\\{}_{}_{}_{}.txt",
        dir,
        name,
        time.tm_year + 1900,
        time.tm_mon + 1,
        time.tm_mday,
    );
    // Create and write to file
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
