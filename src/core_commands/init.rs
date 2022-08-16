use std::process::Command;

pub fn init() {
    let setup_gmork = Command::new("node")
        .arg("-v")
        .spawn()
        .expect("failed to execute");
    setup_gmork.stdout;
}
