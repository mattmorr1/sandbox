use std::process::Command;

#[test]
fn test_commit_count_passes() {
    let output = Command::new("bash")
        .args(["./.autograder/commit_count.sh", "10"])
        .output()
        .expect("Failed to run commit_count.sh");

    assert!(
        output.status.success(),
        "Not enough qualifying commits (minimum 10). Push your work regularly and use feature branches as assigned.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_branch_count_passes() {
    let output = Command::new("bash")
        .args(["./.autograder/branch_count.sh", "6"])
        .output()
        .expect("Failed to run branch_count.sh");

    assert!(
        output.status.success(),
        "Not enough branches with commits (minimum 6). Create and use feature branches as assigned.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
