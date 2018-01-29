use std::collections::{HashMap};
use regex::Regex;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Token {
    KeywordPi,
    KeywordE,
    OpMul,
    OpExp,
    OpDiv,
    OpPlus,
    OpSub,
    Identifier(usize),
    Numeric(f64)
}

pub struct Scanner {
    buf: String,
    pos: usize,
    last_id: usize,
    symbol_table: HashMap<String, usize>,
}
impl Scanner {
    pub fn new(string : String) -> Self {
        Scanner {
            buf: string,
            pos: 0,
            last_id: 0,
            symbol_table: HashMap::new()
        }
    }
    fn lex_identifier(&mut self) -> Option<Token> {
       if let Some(mtch) = (Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap()).find(&self.buf[self.pos..]) {
           let start = self.pos + mtch.start();
           let end = self.pos + mtch.end();
           self.pos = end;
           let id = self.buf[start..end].to_string();
           let id_id = self.symbol_table.entry(id).or_insert(self.last_id);

           if *id_id == self.last_id {
               self.last_id += 1;
           }

           Some(Token::Identifier(*id_id))
       } else {
           None
       }
    }

    pub fn lex_numeric(&mut self) -> Option<Token> {
        if let Some(mtch) = (Regex::new(r"^((?:(?:-[1-9][0-9]*|[0-9][1-9]*)(?:.[0-9]*)))").unwrap()).find(&self.buf[self.pos..]) {

            let start = self.pos + mtch.start();
            let end = self.pos + mtch.end();
            self.pos = end;
            self.buf[start..end].parse().ok().map(|num| Token::Numeric(num)) 
        } else {
            None
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
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

                /* Keywords */

                'p' => {
                    if let Some(mtch) = (Regex::new(r"^pi[^a-zA-Z0-9]").unwrap()).find(&self.buf[self.pos..]) {
                        let end = self.pos + mtch.end();
                        self.pos = end;
                        return Some(Token::KeywordPi);
                    }
                        self.lex_identifier()
                }

                'e' => {
                    if let Some(mtch) = (Regex::new(r"^e[^a-zA-Z0-9]").unwrap()).find(&self.buf[self.pos..]) {
                        let end = self.pos + mtch.end();

                        self.pos = end;
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
}