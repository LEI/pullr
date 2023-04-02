use assert_cmd::Command;

fn bin() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

#[test]
fn fail_without_arg() {
    bin().arg("--dry-run").assert().failure();
}

#[test]
fn success_with_arg() {
    bin().arg("--dry-run").arg("123").assert().success();
}
