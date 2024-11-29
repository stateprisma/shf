use std::{fs, process::Command};

fn main() {
    println!("[start] building frontend");
    let _ = fs::remove_dir_all("public");
    let output = Command::new("pnpm")
        .args(&["build"])
        .current_dir("frontend")
        .output()
        .expect("Failed to execute command");

    println!("[ end ] building frontend");
    if !output.status.success() {
        log_frontend_fail(output.status, &output.stdout, &output.stderr);
        panic!("Frontend build failed");
    }
}

#[inline]
fn log_frontend_fail(status: std::process::ExitStatus, stdout: &[u8], stderr: &[u8]) {
    eprintln!(
        "\nFrontend Build Failed!
======================================
Status Code: {}
======================================
Standard Output:
--------------------------------------
{}
======================================
Standard Error:
--------------------------------------
{}
======================================\n",
        status,
        String::from_utf8_lossy(stdout).trim(),
        String::from_utf8_lossy(stderr).trim(),
    );
}
