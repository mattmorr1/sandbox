use std::process::Command;

#[test]
fn test_rustfmt_passes() {
    let output = Command::new("cargo")
        .args(&["fmt", "--", "--check"])
        .output()
        .expect("Failed to run cargo fmt");

    assert!(
        output.status.success(),
        "Code is not properly formatted. Run 'cargo fmt' to fix.\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn test_clippy_passes() {
    let output = Command::new("cargo")
        .args(&["clippy", "--", "-D", "warnings"])
        .output()
        .expect("Failed to run cargo clippy");

    assert!(
        output.status.success(),
        "Clippy found issues. Fix them before submitting.\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
