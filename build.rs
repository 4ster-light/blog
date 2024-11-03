use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/templates");
    println!("cargo:rerun-if-changed=tailwind.config.js");

    let output = Command::new("bun")
        .args(["run", "build:css"])
        .output()
        .expect("Failed to compile Tailwind CSS");

    if !output.status.success() {
        panic!(
            "Tailwind CSS build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        println!("Tailwind CSS built successfully");
    }
}
