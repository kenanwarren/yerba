use crate::token::{Token, TokenType};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
enum LexicalError {
    #[error("Unexpected character {found:?} found on line {line:?}.")]
    UnexpectedCharacter { found: char, line: usize },
}

#[derive(Clone, Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
  pub fn new(source: String) -> Self {
    Scanner {
        source,
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
    }
  }

  pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
    while !self.is_at_end() {
        self.start = self.current;
        self.scan_token()?;
    }

    self.tokens.push(Token::new(TokenType::Eof, self.line));
    Ok(self.tokens.clone())
  }

  fn scan_token(&mut self) -> Result<(), anyhow::Error> {
    let c = self.advance();
    match c {
        '(' => self.add_token(TokenType::LeftParen),
        ')' => self.add_token(TokenType::RightParen),
        '{' => self.add_token(TokenType::LeftBrace),
        '}' => self.add_token(TokenType::RightBrace),
        ',' => self.add_token(TokenType::Comma),
        '.' => self.add_token(TokenType::Dot),
        '-' => self.add_token(TokenType::Minus),
        '+' => self.add_token(TokenType::Plus),
        ';' => self.add_token(TokenType::Semicolon),
        '*' => self.add_token(TokenType::Star),
        _ => anyhow::bail!(LexicalError::UnexpectedCharacter { found: c, line: self.line }),
    }

    Ok(())
  }

  fn add_token(&mut self, r#type: TokenType) {
    self.tokens.push(Token::new(r#type, self.line));
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source.chars().nth(self.current - 1).unwrap()
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }
}
