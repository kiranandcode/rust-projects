# Language definition
## Lexical definition
### Keywords
pi
e

### Operators
(
)
*
^
/
+
-

### Identifiers
identifier - [a-zA-Z][a-zA-Z0-9]*
numeric    - (-[1-9][0-9]*|[0-9][1-9]*)

## Syntactical definition

expresesion = factor_expression
            | expression + factor_expression
            | expression - factor_expression

factor_expression = primary_expression
                  | primary_expression * factor_expression 
                  | primary_expression / factor_expression
                  | primary_expression ^ factor_expression


primary_expression = identifier
                   | numeric
                   | ( expression )


// example parse

expression -> factor_expression
           -> factor_expression  * primary_expression
           -> primary_expression * primary_expression
           -> numeric            * identifier
           -> 5                  * x



// deriving 1+2*3
expression -> expression + factor_expression
           -> factor_expression + factor_expression
           -> primary_expression + factor_expression
           -> primary_expression + factor_expression * primary_expression
           -> primary_expression + primary_expression * primary_expression
           -> numeric + numeric * numeric
