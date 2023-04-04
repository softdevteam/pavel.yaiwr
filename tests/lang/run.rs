use std::{fs::read_to_string, process::Command};

use lang_tester::LangTester;

static COMMENT_PREFIX: &str = "//";
static LANG_TEST_PATH: &str = "tests/lang/tests";

fn main() {
    LangTester::new()
        .test_dir(LANG_TEST_PATH)
        .test_file_filter(|p| p.extension().unwrap().to_str().unwrap() == "yaiwr")
        // Extract the first sequence of commented line(s) as the tests.
        .test_extract(|p| {
            read_to_string(p)
                .unwrap()
                .lines()
                .skip_while(|l| !l.starts_with(COMMENT_PREFIX))
                .take_while(|l| l.starts_with(COMMENT_PREFIX))
                .map(|l| &l[COMMENT_PREFIX.len()..])
                .collect::<Vec<_>>()
                .join("\n")
        })
        .test_cmds(move |p| {
            let mut runtime = Command::new("cargo");
            runtime.args(&["run", "-q", p.to_str().unwrap()]);
            vec![("Run-time", runtime)]
        })
        .run()
}
