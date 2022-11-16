use std::process::Command;

pub fn play(id: &str) {
    let output = Command::new("mpv")
        .arg(format!("https://www.youtube.com/watch?v={id}").as_str())
        .arg("--no-video")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
