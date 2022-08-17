use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct DefaultCommand {
    announcement: String,
    command: String,
    success_required: bool,
}

pub fn init() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "announcement": "Setting up gmork on your machine",
            "command": "node -v",
            "success_required": false
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let init: DefaultCommand = serde_json::from_str(data)?;

    let split: Vec<&str> = init.command.split_whitespace().collect();
    println!("{:?}", split);

    let setup_gmork = Command::new(split[0])
        .arg(split[1])
        .spawn()
        .expect("failed to execute");
    setup_gmork.stdout;
    Ok(())
}
