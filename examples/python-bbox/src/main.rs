use std::path::Path;

use eyre::{Result, WrapErr};

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::env::set_current_dir(root).wrap_err("failed to set working dir")?;

    let file = Path::new("src").join("main.py");

    let python = which::which("python").unwrap();

    let mut cmd = std::process::Command::new(&python);

    cmd.arg(&file);

    if !cmd
        .status()
        .wrap_err("failed to run python script")?
        .success()
    {
        println!("python script failed");
    }

    Ok(())
}
