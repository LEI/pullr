use std::{
    io::{StdoutLock, Write},
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;

pub(crate) fn command(
    program: &str,
    args: Vec<&str>,
    dir: &Path,
    dry_run: bool,
    out: &mut StdoutLock,
) -> anyhow::Result<()> {
    let prefix = if dry_run { "#" } else { "$" };
    let str = format!("{} {}", program, args.join(" "));
    writeln!(out, "{} {}", prefix, str)?;

    if dry_run {
        return Ok(());
    }

    let mut child = Command::new(program)
        .current_dir(dir)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;

    let res = child
        .wait()
        .with_context(|| format!("error attempting to wait {:?}", str))?;

    if !res.success() {
        return Err(anyhow::Error::msg(format!(
            "failed to execute command {:?}",
            str
        )));
    }

    Ok(())
}
