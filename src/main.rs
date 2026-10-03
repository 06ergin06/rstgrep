use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

enum InputSource {
    File(String),
    Command(String),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len < 3 {
        eprintln!("Usage: rstgrep <pattern> <file>");
        std::process::exit(1);
    }

    let pattern = &args[1];
    let target = &args[2];

    let mut parts = target.split_whitespace();
    let is_command = if let Some(cmd) = parts.next() {
        Command::new(cmd).args(parts).output().is_ok()
    } else {
        false
    };

    let source = if Path::new(target).exists() {
        InputSource::File(target.clone())
    } else if is_command {
        InputSource::Command(target.clone())
    } else {
        eprintln!("Error: target is not file or command.");
        std::process::exit(1);
    };

    let content = match source {
        InputSource::File(path) => fs::read_to_string(&path).expect("File could not be read"),
        InputSource::Command(cmd_str) => {
            let mut parts = cmd_str.split_whitespace();
            let cmd = parts.next().expect("command failed");
            let output = Command::new(cmd)
                .args(parts)
                .output()
                .expect("command failed");
            String::from_utf8_lossy(&output.stdout).to_string()
        }
    };
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(pattern))
        .for_each(|(idx, line)| println!("{}: {line}", idx + 1));
}
