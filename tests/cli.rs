use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;

#[test]
fn illegal_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("dev-cli")?;
    let file = assert_fs::NamedTempFile::new(".dev-cli.yml")?;
    file.write_str("InVaLId YaML $$$")?;

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Docker"));
    Ok(())
}
