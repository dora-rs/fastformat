use eyre::{Result, WrapErr};

use std::path::Path;

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::env::set_current_dir(root).wrap_err("failed to set working dir")?;

    let dataflow = Path::new("dataflow_fastformat.yml");
    destroy_dora()?;
    spawn_dora()?;
    build_dataflow(dataflow)?;
    start_dataflow(dataflow)?;

    let dataflow = Path::new("dataflow_raw.yml");
    start_dataflow(dataflow)?;

    Ok(())
}

fn destroy_dora() -> eyre::Result<()> {
    let dora = which::which("dora").wrap_err("dora not found")?;
    let mut cmd = std::process::Command::new(&dora);

    cmd.arg("destroy");

    if cmd.status().wrap_err("failed to destroy dora")?.success() {
        println!("dora destroyed successfully");
    } else {
        println!("dora destroy failed");
    }

    Ok(())
}

fn spawn_dora() -> eyre::Result<()> {
    let dora = which::which("dora").wrap_err("dora not found")?;
    let mut cmd = std::process::Command::new(&dora);

    cmd.arg("up");

    if cmd.status().wrap_err("failed to spawn dora")?.success() {
        println!("dora spawned successfully");
    } else {
        println!("dora spawn failed");
    }

    Ok(())
}

fn build_dataflow(dataflow: &Path) -> eyre::Result<()> {
    let dora = which::which("dora").wrap_err("dora not found")?;
    let mut cmd = std::process::Command::new(&dora);

    cmd.arg("build");
    cmd.arg("--").arg(dataflow);

    if cmd.status().wrap_err("failed to build dataflow")?.success() {
        println!("dataflow built successfully");
    } else {
        println!("dataflow build failed");
    }

    Ok(())
}

fn start_dataflow(dataflow: &Path) -> eyre::Result<()> {
    let dora = which::which("dora").wrap_err("dora not found")?;
    let mut cmd = std::process::Command::new(&dora);

    cmd.arg("start");
    cmd.arg("--").arg(dataflow);

    if cmd.status().wrap_err("failed to start dataflow")?.success() {
        println!("dataflow executed successfully");
    } else {
        println!("dataflow failed");
    }

    Ok(())
}
