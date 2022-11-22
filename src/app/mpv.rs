use std::{process::Command, thread};

pub fn play(id: &str) {
    // ISSUES: multiple threads can be used -> multiple songs can be played at once

    let id_copy = id.to_owned();

    thread::spawn(move || {
        Command::new("mpv")
            .arg(format!("https://www.youtube.com/watch?v={id_copy}").as_str())
            .arg("--no-video")
            .arg("--no-terminal")
            .arg("--no-config")
            .spawn()
            .expect("failed to execute process");
    });
}
