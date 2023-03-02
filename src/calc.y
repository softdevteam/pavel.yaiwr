%start Opcodes
%avoid_insert "INT"
%expect-unused Unmatched "UNMATCHED"
%%

Opcodes -> Result<Vec<Opcode>, ()>:
    Opcodes Opcode { flattenr($1, $2) }
  | { Ok(vec![]) }
  ;

Opcode -> Result<Opcode, ()>:
      Opcode '+' Term {
        Ok(Opcode::Add{ 
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Term { $1 }
    ;

Term -> Result<Opcode, ()>:
      Term '*' Factor {
        Ok(Opcode::Mul{  
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Factor { $1 }
    ;

Factor -> Result<Opcode, ()>:
      '(' Opcode ')' { $2 }
    | 'INT' { 
        match $1.map_err(|err| format!("Parsing Error: {}", err)) {
            Ok(s) => {
              let s = $lexer.span_str(s.span());
              match s.parse::<u64>() {
                  Ok(n_val) => Ok(Opcode::Number{ value: n_val }),
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
use crate::ast::Opcode;

/// Flatten `rhs` into `lhs`.
fn flattenr<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>) -> Result<Vec<T>, ()> {
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}
