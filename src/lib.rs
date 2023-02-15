use cfgrammar::RIdx;
use lrlex::{lrlex_mod, DefaultLexeme, DefaultLexerTypes, LRNonStreamingLexerDef};
use lrpar::{lrpar_mod, Lexeme, Node};

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

pub fn eval(
    lexerdef: &LRNonStreamingLexerDef<DefaultLexerTypes>,
    input: &str,
) -> Result<String, String> {
    let lexer = lexerdef.lexer(input);
    let (res, errs) = calc_y::parse(&lexer);
    for e in errs {
        println!("{}", e.pp(&lexer, &calc_y::token_epp));
    }
    match res {
        Some(x) => {
            return Ok(format!("{}", Eval::new(input).eval(&x)));
        }
        _ => Err(String::from("Unable to evaluate expression.")),
    }
}

struct Eval<'a> {
    s: &'a str,
}

// stolen from: https://github.dev/softdevteam/grmtools
impl<'a> Eval<'a> {
    fn new(s: &'a str) -> Self {
        return Eval { s };
    }

    fn eval_r_exp(&self, nodes: &Vec<Node<DefaultLexeme, u32>>) -> i64 {
        if nodes.len() == 1 {
            return self.eval(&nodes[0]);
        } else {
            debug_assert_eq!(nodes.len(), 3);
            return self.eval(&nodes[0]) + self.eval(&nodes[2]);
        }
    }

    fn eval_r_term(&self, nodes: &Vec<Node<DefaultLexeme, u32>>) -> i64 {
        if nodes.len() == 1 {
            return self.eval(&nodes[0]);
        } else {
            debug_assert_eq!(nodes.len(), 3);
            return self.eval(&nodes[0]) * self.eval(&nodes[2]);
        }
    }

    fn eval_r_factor(&self, nodes: &Vec<Node<DefaultLexeme, u32>>) -> i64 {
        if nodes.len() == 1 {
            if let Node::Term { lexeme } = nodes[0] {
                self.s[lexeme.span().start()..lexeme.span().end()]
                    .parse()
                    .unwrap()
            } else {
                unreachable!();
            }
        } else {
            debug_assert_eq!(nodes.len(), 3);
            self.eval(&nodes[1])
        }
    }

    fn eval(&self, n: &Node<DefaultLexeme<u32>, u32>) -> i64 {
        match *n {
            Node::Nonterm {
                ridx: RIdx(ridx),
                ref nodes,
            } if ridx == calc_y::R_EXPR => {
                return self.eval_r_exp(nodes);
            }
            Node::Nonterm {
                ridx: RIdx(ridx),
                ref nodes,
            } if ridx == calc_y::R_TERM => {
                return self.eval_r_term(nodes);
            }
            Node::Nonterm {
                ridx: RIdx(ridx),
                ref nodes,
            } if ridx == calc_y::R_FACTOR => {
                return self.eval_r_factor(nodes);
            }
            _ => unreachable!(),
        }
    }
}
