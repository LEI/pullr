use std::{io, path::PathBuf};

use clap::Parser;

use crate::repo::Repo;

/// Pullr
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the local directory.
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Remote repository.
    #[arg(short, long, default_value = "origin")]
    remote: String,

    /// Main branch.
    #[arg(short, long, default_value = "master")]
    branch: String,

    /// Temporary branch.
    #[arg(short, long, default_value = "temp")]
    tmp_branch: String,

    /// Local branch.
    #[arg(short, long, default_value = "pullr")]
    local_branch: String,

    /// Shell command to run if the run was successful.
    #[arg(short, long, default_value = "")]
    command: String,

    /// Enable dry-run mode where available.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// List of pull request IDs.
    pull_requests: Vec<usize>,
}

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let remote = &self.remote;
        let branch = &self.branch;
        let local_branch = &self.local_branch;
        let pull_requests = &self.pull_requests;

        if pull_requests.is_empty() {
            return Err(anyhow::Error::msg("At least one pull request is required").into());
        }

        let stdout = io::stdout();
        let mut out = stdout.lock();

        let repo = Repo::discover(&self.path, self.dry_run)?;

        if self.dry_run {
            log::warn!("DRY-RUN");
        }

        let result = repo.rebase(true, &mut out);
        if result.is_err() {
            // Ignore error
        }
        repo.fetch(remote, &mut out)?;
        repo.checkout(branch, false, &mut out)?;

        let result = repo.delete(local_branch, &mut out);
        if result.is_err() {
            // Ignore error
        }
        repo.checkout(local_branch, true, &mut out)?;
        repo.reset(remote, branch, local_branch, true, &mut out)?;

        for pr in pull_requests {
            repo.fetch_pull_request(remote, pr, &mut out)?;
        }

        for pr in pull_requests {
            repo.add_pull_request(&self.tmp_branch, &self.local_branch, pr, &mut out)?;
        }

        // Install the branch with this command
        // cargo install --locked --path helix-term

        Ok(())
    }
}
