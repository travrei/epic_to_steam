use std::process::{Command, Stdio};
use std::thread;
use std::time;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("ERROR: Needs launch URL and EXE Name");
        return;
    }

    let epic_url = &args[1];
    let exe_name = &args[2];

    println!("Starting url: {}", epic_url);
    Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg(epic_url)
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start process");

    thread::sleep(time::Duration::from_millis(5000));

    let output = Command::new("tasklist")
        .output()
        .expect("Failed to execute command");

    let game_processes = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.contains(exe_name))
        .count();

    if game_processes != 1 {
        println!("Could not find a single process with name: {}", exe_name);
        return;
    }

    println!("Game started.");
}