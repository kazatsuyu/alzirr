
use std::env;
use std::process::Command;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let client_dir = format!("target/{}/client", &profile);
    let args: &[&str] = if profile == "debug" {
        &["make", "src/client/Main.elm"]
    } else {
        &["make", "src/client/Main.elm", "--debug"]
    };
    Command::new("elm").args(args)
                       .arg(&format!("--output={}/index.html", client_dir)).status().unwrap();
}