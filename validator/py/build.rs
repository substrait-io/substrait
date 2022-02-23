use std::process::Command;

fn main() {
    let output = Command::new("python")
        .arg("generate-protos.py")
        .output()
        .expect("failed to generate proto files");

    if !output.status.success() {
        dbg!(output);
        panic!();
    }
}
