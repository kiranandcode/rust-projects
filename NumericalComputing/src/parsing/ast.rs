use std::iter::Peekable;
use std::fmt::{Debug, Display};
use parsing::{Token,Scanner};

#[derive(Debug)]
pub enum PrimaryExpression {
    Identifier(usize),
    Numeric(f64),
    Parens(Box<Expression>),
    KeywordPi,
    KeywordE
}

#[derive(Debug)]
pub enum FactorExpression {
    Base(PrimaryExpression),
    Mult(PrimaryExpression, Box<FactorExpression>),
    Div(PrimaryExpression, Box<FactorExpression>),
    Exp(PrimaryExpression, Box<FactorExpression>),
}

#[derive(Debug)]
pub enum Expression {
    Base(FactorExpression),
    Add(FactorExpression, Box<Expression>),
    Sub(FactorExpression, Box<Expression>),
}

pub fn parse_expression(scanner : &mut Scanner) -> Option<Expression> {
//    println!("Parsing expression - trying to parse factor");
    parse_factor_expression(scanner).and_then(|factor| {
    //    println!("Parsing expression - starting with factor {:?}", factor);
        match scanner.next() {
            Some(tok) => {
                match tok {
                    Token::OpPlus => {
                    //    println!("Parsing expression - plus found");
                        scanner.consume(tok);
                        parse_expression(scanner).map(|expression| {
                            Expression::Add(factor, Box::new(expression))
                        })
                    }
                    Token::OpSub => {
                    //    println!("Parsing expression - minus found");
                        scanner.consume(tok);
                        parse_expression(scanner).map(|expression| {
                        //    println!("Parsing expression - got factor {:?} minus expression {:?}", factor, expression);
                            Expression::Sub(factor, Box::new(expression))
                        })
                    }
                    _ => Some(Expression::Base(factor))
                }
            }
            None => Some(Expression::Base(factor))
        }
    })
}

pub fn parse_factor_expression(scanner : &mut Scanner) -> Option<FactorExpression> {
//    println!("Parsing factor - trying to parse primary");
    parse_primary_expression(scanner).and_then(|primary| {
    //    println!("Parsing factor - starting with primary expression {:?}", primary);
        match scanner.next() {
            Some(tok) => {
                match(tok) {
                    Token::OpMul => {
                    //    println!("Parsing factor - mul found");
                        scanner.consume(tok);
                        parse_factor_expression(scanner).map(|factor| {
                           FactorExpression::Mult(primary, Box::new(factor)) 
                        })
                    }
                    Token::OpDiv => {
                    //    println!("Parsing factor - div found");
                        scanner.consume(tok);
                        parse_factor_expression(scanner).map(|factor| {
                           FactorExpression::Div(primary, Box::new(factor)) 
                        })
                    }
                    Token::OpExp => {
                    //    println!("Parsing factor - exponent found");
                        scanner.consume(tok);
                        parse_factor_expression(scanner).map(|factor| {
                           FactorExpression::Exp(primary, Box::new(factor)) 
                        })
                    }
                    _ => {
                        Some(FactorExpression::Base(primary))
                    }
                }
            }
            None => Some(FactorExpression::Base(primary))
        }
   }) 
}

pub fn parse_primary_expression(scanner : &mut Scanner) -> Option<PrimaryExpression> {
   println!("Parsing primary");
    scanner.next().and_then(|token| {
    //    println!("Parsing primary - got token {:?}", token);
        match token {
            Token::Identifier(pos) => {
               scanner.consume(token.clone());
               Some(PrimaryExpression::Identifier(pos))
            }
            Token::Numeric(val) => {
               scanner.consume(token.clone());
                Some(PrimaryExpression::Numeric(val))
            }
            Token::KeywordE => {
               scanner.consume(token.clone());
               Some(PrimaryExpression::KeywordE)
            }
            Token::KeywordPi => {
               scanner.consume(token.clone());
               Some(PrimaryExpression::KeywordPi)
            }
            Token::LParen => {
                scanner.consume(token);
                parse_expression(scanner).and_then(|expr| {
                    scanner.next().and_then(|should_be_rparen| {
                        match should_be_rparen {
                            Token::RParen => {
                                scanner.consume(should_be_rparen);
                                Some(PrimaryExpression::Parens(Box::new(expr)))
                            }
                            _  => None
                        }
                    })
                })
            }
            _ => None
        }
    })
}