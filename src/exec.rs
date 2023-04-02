use std::{
    io::{StdoutLock, Write},
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context;

pub(crate) fn command(
    program: &str,
    args: &[&str],
    dir: &Path,
    dry_run: bool,
    out: &mut StdoutLock,
) -> anyhow::Result<()> {
    let prefix = if dry_run { " " } else { "$" };
    let command_str = format_command(program, args);
    writeln!(out, "{} {}", prefix, command_str)?;

    if dry_run {
        return Ok(());
    }

    let result = Command::new(program)
        .current_dir(dir)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn: {}", command_str))?
        .wait()
        .with_context(|| format!("error attempting to wait: {}", command_str))?;

    if !result.success() {
        return Err(anyhow::Error::msg(format!(
            "failed to execute command: {}",
            command_str
        )));
    }

    Ok(())
}

fn format_command(program: &str, args: &[&str]) -> String {
    let args_str = args
        .iter()
        .map(|arg| {
            if arg.contains(' ') {
                format!("{:?}", arg)
            } else {
                arg.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    format!("{} {}", program, args_str)
}
