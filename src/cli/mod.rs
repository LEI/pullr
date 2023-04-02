use std::{io, io::Write, path::PathBuf};

use clap::Parser;

use crate::{exec, repo::Repo};

/// CLI arguments are parsed with [clap](https://docs.rs/clap).
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the local directory.
    // FIXME: expand shell to handle tilde
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Remote repository.
    #[arg(short, long, default_value = "origin", help_heading = "Remote")]
    remote: String,

    /// Use "upstream" remote.
    #[arg(
        short = 'u',
        long = "upstream",
        default_value_t = false,
        conflicts_with = "remote",
        help_heading = "Remote"
    )]
    use_upstream: bool,

    /// Main branch.
    #[arg(short, long, default_value = "main", help_heading = "Branch")]
    branch: String,

    /// Use "master" branch.
    // TODO: auto-detect main?
    #[arg(
        short = 'm',
        long = "master",
        default_value_t = false,
        conflicts_with = "branch",
        help_heading = "Branch"
    )]
    use_master: bool,

    /// Temporary branch.
    #[arg(short, long, default_value = "temp", help_heading = "Remote")]
    tmp_branch: String,

    /// Local branch.
    #[arg(short, long, default_value = "pullr", help_heading = "Remote")]
    local_branch: String,

    /// Shell command to run if successful.
    #[arg(short, long)]
    command: Option<String>,

    /// Enable dry-run mode to disable execution.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// Enable verbose mode to explain commands.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// List of pull request IDs, at least one is required.
    #[arg(required = true)]
    pull_requests: Vec<usize>,
}

impl Cli {
    /// Runs the main command.
    pub fn run(&self) -> anyhow::Result<()> {
        let stdout = io::stdout();
        let mut out = stdout.lock();

        if self.verbose {
            writeln!(out, "{:#?}", self)?;
        }

        let remote = if self.use_upstream {
            "upstream"
        } else {
            self.remote.as_str()
        }
        .to_string();

        let branch = if self.use_master {
            "master"
        } else {
            self.branch.as_str()
        }
        .to_string();

        let repo = Repo::discover(&self.path, self.dry_run, self.verbose, &mut out)?;

        if self.dry_run {
            writeln!(out, "# DRY-RUN")?;
        }

        repo.fetch(&remote, &mut out)?;
        repo.checkout(&branch, false, &mut out)?;

        let result = repo.delete(&self.local_branch, &mut out);
        if result.is_err() {
            // Ignore error
        }
        repo.checkout(&self.local_branch, true, &mut out)?;
        repo.reset(&remote, &branch, &self.local_branch, true, &mut out)?;

        for pr in &self.pull_requests {
            repo.fetch_pull_request(&remote, pr, &mut out)?;
        }

        for pr in &self.pull_requests {
            repo.add_pull_request(&self.tmp_branch, &self.local_branch, pr, &mut out)?;
        }

        if let Some(command) = &self.command {
            exec::command(
                "sh",
                &["-c", command.as_str()],
                &self.path,
                self.dry_run,
                &mut out,
            )?;
        }

        Ok(())
    }
}
