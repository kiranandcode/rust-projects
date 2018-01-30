use std::collections::HashMap;
use std::fmt::{Debug, Display};
use parsing::ast::{Expression, PrimaryExpression, FactorExpression};
use parsing::ast::parse_expression;
use parsing::Scanner;

#[derive(Debug, Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Identifier(usize),
    Numeric(f64),
    Pow(Box<Expr>, Box<Expr>),
    Ln(Box<Expr>),
    E
}

#[derive(Debug)]
pub struct Formula {
    symbol_list : Vec<String>,
    symbol_table: HashMap<String, usize>,
    expression: Expr
}

impl Formula {
    pub fn from_scanner(mut scanner : Scanner) -> Option<Self> {
       parse_expression(&mut scanner).map(|expression| {
            Formula {
                symbol_list: scanner.symbols,
                symbol_table: scanner.symbol_table,
                expression: Expr::from_ast_expression(expression)
            }
       })
   }

    pub fn eval(valuation : &HashMap<String, f64>) -> f64 {
       0.0
    }
}

impl Expr {
    fn from_ast_primary(ast : PrimaryExpression) -> Self {
        match ast {
            PrimaryExpression::Identifier(id) => Expr::Identifier(id),
            PrimaryExpression::Numeric(val)   => Expr::Numeric(val),
            PrimaryExpression::KeywordPi      => Expr::Numeric(3.1415),
            PrimaryExpression::KeywordE       => Expr::E,
            PrimaryExpression::Parens(expr)   => Self::from_ast_expression(*expr)
        }
    }

    fn from_ast_factor(ast : FactorExpression) -> Self {
        match ast {
            FactorExpression::Base(expr) => Self::from_ast_primary(expr),
            FactorExpression::Exp(primary, expr)  => {
                let result = Self::from_ast_primary(primary);
                match result {
                    Expr::E => Expr::Pow(
                        Box::new(Expr::E),
                        Box::new(
                            Self::from_ast_factor(*expr)
                        )
                    ),
                    Expr::Identifier(id) => {
                        Expr::Pow(
                            Box::new(Expr::Identifier(id)),
                            Box::new(Self::from_ast_factor(*expr))
                        )
                    }
                    _ => {
                        Expr::Pow(
                            Box::new(Expr::E),
                            Box::new(Expr::Mult(
                                Box::new(Expr::Ln(Box::new(result))),
                                Box::new(Self::from_ast_factor(*expr))
                            ))
                        )
                    }
                }
           }
           FactorExpression::Mult(primary, expr) => {
               let result_primary = Self::from_ast_primary(primary);
               let result_factory = Self::from_ast_factor(*expr);
               if let Expr::Numeric(val) = result_primary {
                   if let Expr::Numeric(other) = result_factory {
                       Expr::Numeric(val * other)
                   } else {
                       Expr::Mult(Box::new(result_primary), Box::new(result_factory))
                   }
               } else {
                    Expr::Mult(Box::new(result_primary), Box::new(result_factory))
               }
           }
           FactorExpression::Div(primary, expr) => {
               let result_primary = Self::from_ast_primary(primary);
               let result_factory = Self::from_ast_factor(*expr);
               if let Expr::Numeric(val) = result_primary {
                   if let Expr::Numeric(other) = result_factory {
                       Expr::Numeric(val / other)
                   } else {
                       Expr::Div(Box::new(result_primary), Box::new(result_factory))
                   }
               } else {
                    Expr::Div(Box::new(result_primary), Box::new(result_factory))
               }
           }
 
        }
    }

   pub fn from_ast_expression(expr : Expression) -> Self {
        match expr {
            Expression::Base(factr) => {
                Self::from_ast_factor(factr)
            }
            Expression::Add(factr, expr) => {
                let result_factr = Self::from_ast_factor(factr);
                let result_expr  = Self::from_ast_expression(*expr);
                if let Expr::Numeric(val) = result_factr {
                    if let Expr::Numeric(other) = result_expr {
                        Expr::Numeric(val + other)
                    } else {
                        Expr::Add(Box::new(result_factr), Box::new(result_expr))
                    }
                } else {
                    Expr::Add(Box::new(result_factr), Box::new(result_expr))
                }
            }
            Expression::Sub(factr, expr) => {
                let result_factr = Self::from_ast_factor(factr);
                let result_expr  = Self::from_ast_expression(*expr);
                if let Expr::Numeric(val) = result_factr {
                    if let Expr::Numeric(other) = result_expr {
                        Expr::Numeric(val - other)
                    } else {
                        Expr::Sub(Box::new(result_factr), Box::new(result_expr))
                    }
                } else {
                    Expr::Sub(Box::new(result_factr), Box::new(result_expr))
                }
            }
 
        }
    }




}