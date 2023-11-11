use clap::{Command, arg};
use std::{io::{self, Write}, fs::read_to_string};
use anyhow::Result;

fn main() -> Result<()> {
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
            run_file(file)
        }
        Some(("repl", _)) => run_repl(),
        _ => unreachable!("we've goofed up"),
    }
}

fn run_file(file: &String) -> Result<()> {
    for line in read_to_string(file)?.lines() {
        run(line.to_string())?;
    }
    Ok(())
}

fn run_repl() -> Result<()> {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim().is_empty() {
            break;
        }
        run(buffer)?;
    }
    Ok(())
}

fn run(source: String) -> Result<()> {
    let chars: Vec<char> = source
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    for c in chars {
        println!("{}", c);
    }
    Ok(())
}
