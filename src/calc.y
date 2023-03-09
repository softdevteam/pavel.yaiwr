%start Expr
%avoid_insert "INTEGER"
%expect-unused Unmatched "UNMATCHED"
%%

Expr -> Result<AstNode, ()>:
    Expr "ADD" Term { Ok(AstNode::Add{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Term { $1 } 
    |  "PRINT_LN" "(" Expr ")" { Ok(AstNode::PrintLn{ rhs: Box::new($3?) }) }
    ;

Term -> Result<AstNode, ()>:
      Term 'MUL' Factor { Ok(AstNode::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Factor { $1 }
    ;

Factor -> Result<AstNode, ()>:
    "(" Expr ")" { $2 }
    | "INTEGER" { 
        match $1.map_err(|err| format!("Parsing Error: {}", err)) {
            Ok(s) => {
              let s = $lexer.span_str(s.span());
              match s.parse::<u64>() {
                  Ok(n_val) => Ok(AstNode::Number{ value: n_val }),
                  Err(_) => Err(())
              }
            }
            Err(_) => Err(())
        }
      }
    ;

Unmatched -> ():
      "UNMATCHED" { };
%%
use crate::ast::AstNode;
