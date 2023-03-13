#[cfg(test)]
mod tests {
    use std::process::Command;

    use yaiwr::Calc;

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
    fn test_test2() {
        let cmd = "'fun _a(){println(2);};_a();'";
        let output = Command::new("cargo")
            .arg("run")
            .arg(cmd)
            .output()
            .expect(format!("comand 'cargo run {}' failed", cmd).as_str());

        assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n",);
    }

    #[test]
    fn test_test() {
        use std::process::Command;
        let output = Command::new("cargo")
            .arg("run")
            .arg("programs/functions.yaiwr")
            .output()
            .expect("command 'cargo run programs/functions.yaiwr' failed");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "4\n");
    }
}
