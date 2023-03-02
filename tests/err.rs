#[cfg(test)]
mod tests {
    use yaiwr::Calc;

    #[test]
    fn eval_error() {
        let parsing_err = Calc::from_str("invalid input").err().unwrap();
        assert_ne!(parsing_err, "")
    }
}
