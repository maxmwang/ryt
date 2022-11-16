use std::process::Command;

pub fn play(id: &str) {
    Command::new("mpv")
        .arg(format!("https://www.youtube.com/watch?v={id}").as_str())
        .arg("--no-video")
        .output()
        .expect("failed to execute process");
}
