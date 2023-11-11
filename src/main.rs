use clap::{Command, arg};
use std::{io::{self, Write}, fs::read_to_string};

fn main() {
    let cmd = Command::new("yerba")
        .bin_name("yerba")
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
            .about("Runs yerba file")
            .arg(arg!(<FILE> "The file to run"))
            .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("repl")
            .about("Starts a yerba repl"),
        );
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let file = sub_matches.get_one::<String>("FILE").expect("required");
            run_file(file);
        }
        Some(("repl", _)) => run_repl(),
        _ => unreachable!("we've goofed up"),
    };
}

fn run_file(file: &String) {
    for line in read_to_string(file).unwrap().lines() {
        run(line.to_string());
    }
}

fn run_repl() {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().is_empty() {
            break;
        }
        run(buffer);
    }
}

fn run(source: String) {
    let chars: Vec<char> = source
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    for c in chars {
        println!("{}", c);
    }
}
