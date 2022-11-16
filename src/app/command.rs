use std::process::Command;
use std::thread;

pub fn play(id: &String) {
    // ISSUES: multiple threads can be used -> multiple songs can be played at once

    let id_copy = id.clone();

    thread::spawn(move || {
        Command::new("mpv")
            .arg(format!("https://www.youtube.com/watch?v={id_copy}").as_str())
            .arg("--no-video")
            .output()
            .expect("failed to execute process");
    });
}
