use std::collections::HashMap;
use std::fmt::{Debug, Display};
use parsing::ast::{Expression, PrimaryExpression, FactorExpression};
use parsing::ast::parse_expression;
use parsing::Scanner;
use std::f64::consts;

#[macro_export]
macro_rules! valuation {
    (
        $( $key:expr => $value:expr),*
    ) => {{
        use std::collections::HashMap;
        let mut hashmap : HashMap<String, f64> = HashMap::new();
        $( hashmap.insert($key.to_owned(), $value); )*
        hashmap 
    }}
}

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

    pub fn eval(&self, valuation : &HashMap<String, f64>) -> f64 {
        eval_expr(&self.expression, &self.symbol_list, valuation)
    }
}

fn eval_expr(expr: &Expr, symbols : &Vec<String>, valuation : &HashMap<String, f64>) -> f64 {
    match expr {
        &Expr::Identifier(id) => *valuation.get(&symbols[id]).expect("Valuation not provided"),
        &Expr::E              => consts::E,
        &Expr::Numeric(val)   => val,
        &Expr::Add(ref exprA, ref exprB) => {
            let valexprA = eval_expr(&**exprA, symbols, valuation);
            let valexprB = eval_expr(&**exprB, symbols, valuation);
            valexprA + valexprB
        }
        &Expr::Sub(ref exprA, ref exprB) => {
            let valexprA = eval_expr(&**exprA, symbols, valuation);
            let valexprB = eval_expr(&**exprB, symbols, valuation);
            valexprA - valexprB
        }
        &Expr::Pow(ref exprA, ref exprB) => {
            let valexprA = eval_expr(&**exprA, symbols, valuation);
            let valexprB = eval_expr(&**exprB, symbols, valuation);
            valexprA.powf(valexprB)
        }
        &Expr::Mult(ref exprA, ref exprB) => {
            let valexprA = eval_expr(&**exprA, symbols, valuation);
            let valexprB = eval_expr(&**exprB, symbols, valuation);
            valexprA * valexprB
        }
        &Expr::Div(ref exprA, ref exprB) => {
            let valexprA = eval_expr(&**exprA, symbols, valuation);
            let valexprB = eval_expr(&**exprB, symbols, valuation);
            valexprA / valexprB
        }
        &Expr::Ln(ref exprA) => {
            let valexprA = eval_expr(&**exprA, symbols, valuation);
            valexprA.ln()
        }
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