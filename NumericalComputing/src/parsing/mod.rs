use std::collections::{HashMap};
use regex::Regex;
use std::fmt::{Debug, Display};
pub mod ast;

#[derive(Debug,Clone, PartialEq)]
pub enum Token {
    KeywordPi,
    KeywordE,
    OpMul,
    OpExp,
    OpDiv,
    OpPlus,
    OpSub,
    LParen,
    RParen,
    Identifier(usize),
    Numeric(f64)
}

pub struct Scanner {
    buf: String,
    pos: usize,
    pub symbols: Vec<String>,
    pub symbol_table: HashMap<String, usize>,
    last_tok: Option<Token>,
}

impl Scanner {
    pub fn new(string : String) -> Self {
        Scanner {
            buf: string,
            pos: 0,
            symbols: Vec::new(),
            symbol_table: HashMap::new(),
            last_tok: None
        }
    }
    fn lex_identifier(&mut self) -> Option<Token> {
       if let Some(mtch) = (Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap()).find(&self.buf[self.pos..]) {
           let start = self.pos + mtch.start();
           let end = self.pos + mtch.end();
           self.pos = end;
           let id = self.buf[start..end].to_string();
           let id_id = self.symbol_table.entry(id.clone()).or_insert(self.symbols.len());

           if *id_id == self.symbols.len() {
               self.symbols.push(id);
           }

           Some(Token::Identifier(*id_id))
       } else {
           None
       }
    }

    pub fn lex_numeric(&mut self) -> Option<Token> {
        // println!("parsing numeric");
        if let Some(mtch) = (Regex::new(r"^((?:(?:-[1-9][0-9]*|[1-9][0-9]*|0)(?:\.[0-9][0-9]*)))").unwrap()).find(&self.buf[self.pos..]) {

            let start = self.pos + mtch.start();
            let end = self.pos + mtch.end();
            self.pos = end;
            self.buf[start..end].parse().ok().map(|num| Token::Numeric(num)) 
        } else {
            None
        }
    }

   fn internal_next(&mut self) -> Option<Token> {
        // skip whitespace
        if let Some(mtch) = (Regex::new(r"^\s*").unwrap()).find(&self.buf[self.pos..]) {
            let o = mtch.end();
            self.pos += o;
        }
        // end of string
        if self.pos >= self.buf.len() {
            return None;
        }
        if let Some(next_chr) = self.buf[self.pos..].chars().next() {

            match next_chr {
                /* Operators */
                '*'  => {
                    self.pos += 1;
                    Some(Token::OpMul)
                }
                '/'  => {
                    self.pos += 1;
                    Some(Token::OpDiv)
                }
                '+'  => {
                    self.pos += 1;
                    Some(Token::OpPlus)
                }
                '-'  => {
                    self.pos += 1;
                    Some(Token::OpSub)
                }
                '^'  => {
                    self.pos += 1;
                    Some(Token::OpExp)
                }
                '('  => {
                    self.pos += 1;
                    Some(Token::LParen)
                }
                ')'  => {
                    self.pos += 1;
                    Some(Token::RParen)
                }

                /* Keywords */

                'p' => {
                    if let Some(mtch) = (Regex::new(r"^pi[^a-zA-Z0-9]").unwrap()).find(&self.buf[self.pos..]) {
                        let end = self.pos + mtch.end();
                        self.pos = end-1;
                        return Some(Token::KeywordPi);
                    }
                        self.lex_identifier()
                }

                'e' => {
                    if let Some(mtch) = (Regex::new(r"^e[^a-zA-Z0-9]").unwrap()).find(&self.buf[self.pos..]) {
                        let end = self.pos + mtch.end();

                        self.pos = end-1;
                        return Some(Token::KeywordE);
                    }
                    self.lex_identifier()
                }
                x if x.is_alphabetic() => self.lex_identifier(),
                x if x.is_numeric()    => self.lex_numeric(),
                _                      => None
            }
        } else {
            None
        }
 
   }

    pub fn consume(&mut self, token : Token) {
        match self.last_tok.take() {
            Some(last_token) => {
                if last_token != token {
                    panic!("Tried to consume the wrong token have: {:?}, given: {:?}", last_token, token);
                }
            }
            None => panic!("Tried to consume an unknown token!")
        }
    }

}


impl Iterator for Scanner {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // println!("Next called with remaining string {:?}", &self.buf[self.pos..]);
        // println!("last token is {:?}", self.last_tok);
        if self.last_tok.is_none() {
            self.last_tok = self.internal_next();
            // println!("retrieving next token to be {:?}", self.last_tok);
        } 
        return self.last_tok.clone();
   }

}