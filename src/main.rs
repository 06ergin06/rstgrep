use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

enum InputSource {
    File(String),
    Command,
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
    let mut command_output = None;

    let source = if Path::new(target).exists() {
        InputSource::File(target.clone())
    } else {
        let mut parts = target.split_whitespace();
        if let Some(cmd) = parts.next() {
            command_output = Command::new(cmd).args(parts).output().ok();
        }

        if command_output.is_some() {
            InputSource::Command
        } else {
            eprintln!("Error: target is not file or command.");
            std::process::exit(1);
        }
    };

    let content = match source {
        InputSource::File(path) => fs::read_to_string(&path).expect("File could not be read"),
        InputSource::Command => {
            let out = command_output.expect("command output missing");
            String::from_utf8_lossy(&out.stdout).to_string()
        }
    };
    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| re.is_match(line))
        .for_each(|(idx, line)| println!("{}: {line}", idx + 1));
}
