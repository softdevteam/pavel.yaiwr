#[derive(Debug)]
pub enum Instruction {
    Add,
    Mul,
    Push { value: u64 },
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Instruction) -> bool {
        match (self, other) {
            (Instruction::Add, Instruction::Add) => true,
            (Instruction::Mul, Instruction::Mul) => true,
            (Instruction::Push { value: self_val }, Instruction::Push { value: other_val }) => {
                self_val == other_val
            }
            _ => false,
        }
    }
}
