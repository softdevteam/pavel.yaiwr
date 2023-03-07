#[cfg(test)]
mod tests {
    use core::panic;

    use yaiwr::{instruction::Instruction, Calc};

    #[test]
    fn bytecode() {
        let ast = Calc::from_str("1*2").unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        match bytecode.as_slice() {
            [first, second, third] => {
                assert_eq!(first, &Instruction::Push { value: 1 });
                assert_eq!(second, &Instruction::Push { value: 2 });
                assert_eq!(third, &Instruction::Mul);
            }
            _ => panic!("expected bytecodes to be not empty!"),
        }
    }

    #[test]
    #[should_panic(expected = "overflowed")]
    fn add_overflow_max_u64() {
        let input = format!("{}+{}", u64::MAX, 1);
        let ast = Calc::from_str(input.as_str()).unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        Calc::eval(bytecode).unwrap();
    }

    #[test]
    fn add_no_overflow() {
        let input = format!("{}+{}", u64::MAX - 1, 1);
        let ast = Calc::from_str(input.as_str()).unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        Calc::eval(bytecode).unwrap();
    }

    #[test]
    #[should_panic(expected = "overflowed")]
    fn mul_overflow() {
        let input = format!("{}*{}", u64::MAX, 2);
        let ast = Calc::from_str(input.as_str()).unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        Calc::eval(bytecode).unwrap();
    }

    #[test]
    fn mul_no_overflow() {
        let input = format!("{}*{}", u64::MAX, 1);
        let ast = Calc::from_str(input.as_str()).unwrap();
        let bytecode = &mut vec![];
        Calc::to_bytecode(ast, bytecode);
        Calc::eval(bytecode).unwrap();
    }
}
