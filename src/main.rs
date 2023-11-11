use clap::{Command, arg};
use std::{io::{self, Write}, fs::read_to_string, fmt::{self}};
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

#[derive(Clone, Debug)]
enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenType::*;
        match self {
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            Semicolon => write!(f, ";"),
            Slash => write!(f, "/"),
            Star => write!(f, "*"),
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Identifier(s) => write!(f, "{}", s),
            String(s) => write!(f, "\"{}\"", s),
            Number(num) => write!(f, "{}", num),
            And => write!(f, "and"),
            Class => write!(f, "class"),
            Else => write!(f, "else"),
            False => write!(f, "false"),
            Fun => write!(f, "fun"),
            For => write!(f, "for"),
            If => write!(f, "if"),
            Nil => write!(f, "nil"),
            Or => write!(f, "or"),
            Print => write!(f, "print"),
            Return => write!(f, "return"),
            Super => write!(f, "super"),
            This => write!(f, "this"),
            True => write!(f, "true"),
            Var => write!(f, "var"),
            While => write!(f, "while"),
            Eof => write!(f, "EOF"),
        }
    }
}

#[derive(Clone, Debug)]
struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn new(r#type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            line,
            literal: "".to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r#type, self.lexeme, self.literal)
    }
}
