use std::process::Command;

// Stamps local build identity into the binary so the startup banner can show
// exactly which build is running. The debug-build pipeline injects
// GOOSE_BUILD_NUMBER / GOOSE_BUILD_TIME (and bumps the number every run); a
// plain `cargo build` falls back to "dev" with the live git sha/branch.
fn main() {
    let build_number = std::env::var("GOOSE_BUILD_NUMBER").unwrap_or_else(|_| "dev".to_string());
    let build_time = std::env::var("GOOSE_BUILD_TIME").unwrap_or_else(|_| "unknown".to_string());
    let sha = git(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let branch = std::env::var("GOOSE_BUILD_BRANCH")
        .ok()
        .or_else(|| git(&["rev-parse", "--abbrev-ref", "HEAD"]))
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GOOSE_BUILD_NUMBER={build_number}");
    println!("cargo:rustc-env=GOOSE_BUILD_TIME={build_time}");
    println!("cargo:rustc-env=GOOSE_BUILD_SHA={sha}");
    println!("cargo:rustc-env=GOOSE_BUILD_BRANCH={branch}");

    println!("cargo:rerun-if-env-changed=GOOSE_BUILD_NUMBER");
    println!("cargo:rerun-if-env-changed=GOOSE_BUILD_TIME");
    println!("cargo:rerun-if-env-changed=GOOSE_BUILD_BRANCH");
}

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (!s.is_empty()).then_some(s)
}
