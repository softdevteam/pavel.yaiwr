%start Expr
%%

Expr -> Result<i128, InterpError>:
      Expr '+' Term { Ok($1? + $3?) }
    | Term { $1 }
    ;

Term -> Result<i128, InterpError>:
      Term '*' Factor { Ok($1? * $3?) }
    | Factor { $1 }
    ;

Factor -> Result<i128, InterpError>:
      '(' Expr ')' { $2 }
    | 'INT'
      {
          let v = $1.map_err(|_| ())?;
          parse_int($lexer.span_str(v.span()))
      }
    ;
%%
// Any functions here are in scope for all the grammar actions above.

pub enum InterpError {
    InvalidInput(String),
    NotImplememented,
}

// Was forced to implmented From trait for Factor implemntation.
impl From<()> for InterpError {
    fn from(_: ()) -> Self {
        InterpError::NotImplememented
    }
}

impl InterpError {
    fn to_string(&self) -> String {
        match self {
            InterpError::InvalidInput(desc) => String::from(desc),
            InterpError::NotImplememented => String::from("not implemented"),
        }
    }
}

fn parse_int(s: &str) -> Result<i128, InterpError> {
    match s.parse::<i128>() {
        Ok(val) => Ok(val),
        Err(err) => Err(InterpError::InvalidInput(format!(
            "Error! {}. String '{}' cannot be represented as a i128.",
            err.to_string(),
            s
        ))),
    }
}
