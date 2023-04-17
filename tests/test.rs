// #[cfg(test)]

// mod tests {
//     use yaiwr::{
//         instruction::{EvalResult, StackValue},
//         Calc,
//     };

//     #[test]
//     fn lalala() {
//         assert_eq!(
//             Calc::eval_input("
//             let g_var = 1;

// fun f1 (){
//     g_var = 2;

//     return 2;
// }

// println(f1() + g_var);
// ".to_string()).unwrap().unwrap(),
//             EvalResult::Value(StackValue::Integer(8)),
//             "expected 2*3+2=8"
//         );
//     }
// }
