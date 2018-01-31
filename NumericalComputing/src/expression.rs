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

#[derive(Debug, Clone)]
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

    pub fn single_variable_eval(&self, valuation : f64) -> f64 {
        if self.symbol_list.len() != 1 {
            panic!("running single variable eval on function with more than single variable");
        }
        eval_expr_single(&self.expression, valuation)
    }

    pub fn derive(&self, wrt : &str) -> Self {
        let wrt = self.symbol_table.get(wrt).expect("deriving with respect to unknown value");
        Formula{
            symbol_list: self.symbol_list.clone(),
            symbol_table: self.symbol_table.clone(),
            expression: derive_expr(&self.expression, *wrt)
        }
    }

    pub fn single_variable_derive(&self) -> Self {
        if self.symbol_list.len() != 1 {
            panic!("running single variable eval on function with more than single variable");
        }
 
        Formula{
            symbol_list: self.symbol_list.clone(),
            symbol_table: self.symbol_table.clone(),
            expression: derive_expr(&self.expression, 0)
        }
    }
}

fn derive_expr(expr : &Expr, wrt : usize) -> Expr {
    match expr {
        &Expr::Add(ref exprA, ref exprB) => {
            Expr::Add(
                Box::new(derive_expr(&**exprA, wrt)),
                Box::new(derive_expr(&**exprB, wrt))
            )
        }
        &Expr::Sub(ref exprA, ref exprB) => {
            Expr::Sub(
                Box::new(derive_expr(&**exprA, wrt)),
                Box::new(derive_expr(&**exprA, wrt))
            )
        }
        &Expr::Identifier(id) => {
            // d/dx(x) = 1, d/dx(y) = 0
            if id == wrt {
                Expr::Numeric(1.0)
            } else {
                Expr::Numeric(0.0)
            }
        }
        &Expr::Numeric(_) => {
            Expr::Numeric(0.0)
       }
       &Expr::E => {
           Expr::Numeric(0.0)
       }
       &Expr::Mult(ref exprA, ref exprB) => {
           // d/dx(f(x)g(x)) = d/dx(f(x))g(x) + d/dx(g(x))f(x)
            Expr::Add(
                Box::new(
                    Expr::Mult(
                        Box::new(derive_expr(&**exprA, wrt)),
                        exprB.clone()
                    )
                ),
                Box::new(
                    Expr::Mult(
                        Box::new(derive_expr(&**exprB, wrt)),
                        exprA.clone()
                    )
 
                )
            )
        }
        &Expr::Pow(ref exprA, ref exprB) => {
            match &**exprA {
                &Expr::E => Expr::Mult(
                    Box::new(derive_expr(&**exprB, wrt)),
                    Box::new(Expr::Pow(Box::new(Expr::E), exprB.clone()))
                ),
                expr => {
                    match &**exprB {
                        &Expr::Numeric(val) => Expr::Mult(
                            Box::new(
                                Expr::Numeric(val)
                            ),
                            Box::new(Expr::Mult(
                                Box::new(derive_expr(expr,wrt)),
                                Box::new(Expr::Pow(
                                    Box::new(expr.clone()),
                                    Box::new(Expr::Numeric(val-1.0))
                                ))
                            ))
                        ),
                        &Expr::Identifier(id) => Expr::Mult(
                            Box::new(
                                Expr::Identifier(id)
                            ),
                            Box::new(Expr::Mult(
                                Box::new(derive_expr(expr, wrt)),
                                Box::new(Expr::Pow(
                                    Box::new(expr.clone()),
                                    Box::new(Expr::Sub(
                                        Box::new(Expr::Identifier(id)),
                                        Box::new(Expr::Numeric(1.0))
                                    ))
                                ))
                            ))
                        ),
                        &Expr::E => Expr::Mult(
                            Box::new(
                                Expr::E
                            ),
                            Box::new(Expr::Mult(
                                Box::new(derive_expr(expr, wrt)),
                                Box::new(Expr::Pow(
                                    Box::new(expr.clone()),
                                    Box::new(Expr::Sub(
                                        Box::new(Expr::E),
                                        Box::new(Expr::Numeric(1.0))
                                    ))
                                ))
                            ))
                        ),
                        exponent => {
                            let reformatted = Expr::Pow(Box::new(Expr::E), Box::new(
                                Expr::Mult(
                                    Box::new(Expr::Ln(Box::new(expr.clone()))),
                                    Box::new(exponent.clone())
                                )
                            ));
                            derive_expr(&reformatted, wrt)
                        }
                    }
                }
            }
        }
        &Expr::Ln(ref exprA) => {
            Expr::Mult(
                Box::new(derive_expr(&**exprA, wrt)),
                Box::new(Expr::Pow(
                    exprA.clone(),
                    Box::new(Expr::Numeric(-1.0))
                ))
            )
        }
        &Expr::Div(ref exprA, ref exprB) => {
           // d/dx(f(x)/g(x)) = d/dx(f(x))g(x) - d/dx(g(x))f(x) / g(x) ^ 2
           Expr::Div(
            Box::new(Expr::Sub(
                Box::new(
                    Expr::Mult(
                        Box::new(derive_expr(&**exprB, wrt)),
                        exprA.clone()
                    )
                ),
                Box::new(
                    Expr::Mult(
                        Box::new(derive_expr(&**exprA, wrt)),
                        exprB.clone()
                    )
 
                )
            )),
            Box::new(
                Expr::Pow(
                    exprB.clone(),
                    Box::new(
                        Expr::Numeric(2.0)
                    )
                )
            )
           )
        }
 

    }
}

fn eval_expr_single(expr: &Expr, x : f64) -> f64 {
    match expr {
        &Expr::Identifier(id) => x,
        &Expr::E              => consts::E,
        &Expr::Numeric(val)   => val,
        &Expr::Add(ref exprA, ref exprB) => {
            let valexprA = eval_expr_single(&**exprA, x);
            let valexprB = eval_expr_single(&**exprB, x);
            valexprA + valexprB
        }
        &Expr::Sub(ref exprA, ref exprB) => {
            let valexprA = eval_expr_single(&**exprA, x);
            let valexprB = eval_expr_single(&**exprB, x);
            valexprA - valexprB
        }
        &Expr::Pow(ref exprA, ref exprB) => {
            let valexprA = eval_expr_single(&**exprA, x);
            let valexprB = eval_expr_single(&**exprB, x);
            valexprA.powf(valexprB)
        }
        &Expr::Mult(ref exprA, ref exprB) => {
            let valexprA = eval_expr_single(&**exprA, x);
            let valexprB = eval_expr_single(&**exprB, x);
            valexprA * valexprB
        }
        &Expr::Div(ref exprA, ref exprB) => {
            let valexprA = eval_expr_single(&**exprA, x);
            let valexprB = eval_expr_single(&**exprB, x);
            valexprA / valexprB
        }
        &Expr::Ln(ref exprA) => {
            let valexprA = eval_expr_single(&**exprA, x);
            valexprA.ln()
        }
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