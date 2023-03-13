#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Add,
    Mul,
    Push {
        value: u64,
    },
    PrintLn,
    Assign {
        id: String,
    },
    Load {
        id: String,
    },
    Function {
        id: String,
        params: Vec<String>,
        body: Vec<Instruction>,
    },
    FunctionCall {
        id: String,
        args: Vec<Vec<Instruction>>,
    },
}
