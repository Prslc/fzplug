use std::io::Write;
use std::process::{Command, Stdio};

/// Launch fuzzel in dmenu mode and return user selection
pub fn dmenu(prompt: &str, input_text: &str) -> Option<String> {
    let mut child = Command::new("fuzzel")
        .arg("--dmenu")
        .arg("--prompt")
        .arg(prompt)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;

    // Write options to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input_text.as_bytes()).ok()?;
    }

    // Wait for fuzzel to finish and read output
    let output = child.wait_with_output().ok()?;
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    } else {
        None
    }
}
