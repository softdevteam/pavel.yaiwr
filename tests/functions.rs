#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn var_test_file_expect_output_1984() {
        let cmd = "'fun _some (){ 2+2 }'";
        let output = Command::new("cargo")
            .arg("run")
            .arg(cmd)
            .output()
            .expect(format!("comand 'cargo run {}' failed", cmd).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "",);
    }

    #[test]
    fn test_functions_args() {
        use std::process::Command;
        let output = Command::new("cargo")
            .arg("run")
            .arg("programs/tests/functions_expect_output_15.yaiwr")
            .output()
            .expect("command 'cargo run programs/tests/functions_expect_output_15.yaiwr' failed");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "15\n");
    }
}
