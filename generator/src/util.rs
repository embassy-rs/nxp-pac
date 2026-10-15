use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::bail;

/// Perform rustfmt on a single file.
pub fn rustfmt(path: &Path) -> anyhow::Result<()> {
    let output = Command::new("rustfmt")
        .args(["--edition", "2024", "--config", "newline_style=Unix"])
        .arg(path.canonicalize()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "Error during rustfmt: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
