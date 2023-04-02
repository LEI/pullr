use std::{
    io::{StdoutLock, Write},
    path::{Path, PathBuf},
};

use crate::exec;

#[derive(Debug)]
pub(crate) struct Repo {
    dry_run: bool,
    verbose: bool,
    work_dir: PathBuf,
}

impl Repo {
    pub(crate) fn discover(
        path: &Path,
        dry_run: bool,
        verbose: bool,
        out: &mut StdoutLock,
    ) -> anyhow::Result<Self> {
        let work_dir = path.to_path_buf();
        writeln!(out, "# Working directory {:?}", work_dir.display())?;

        exec::command(
            "git",
            &["diff", "--no-ext-diff", "--quiet"],
            &work_dir,
            dry_run,
            out,
        )?;

        let repo = Self {
            dry_run,
            verbose,
            work_dir,
        };

        Ok(repo)
    }

    pub(crate) fn fetch(&self, remote: &str, out: &mut StdoutLock) -> anyhow::Result<()> {
        if self.verbose {
            writeln!(out, "# Fetching remote {:?}", remote)?;
        }

        self.git(&["fetch", remote], out)
    }

    pub(crate) fn checkout(
        &self,
        branch: &str,
        create: bool,
        out: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        if self.verbose {
            writeln!(
                out,
                "# Checking out branch {:?} (create: {})",
                branch, create
            )?;
        }

        let mut args = vec!["checkout"];
        if create {
            args.push("-b");
        }
        args.push(branch);
        self.git(&args, out)?;

        Ok(())
    }

    pub(crate) fn delete(&self, branch: &str, out: &mut StdoutLock) -> anyhow::Result<()> {
        if self.verbose {
            writeln!(out, "# Deleting branch {:?}", branch)?;
        }

        self.git(&["branch", "-D", branch], out)?;

        Ok(())
    }

    pub(crate) fn reset(
        &self,
        remote: &str,
        branch: &str,
        local_branch: &str,
        hard: bool,
        out: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        if self.verbose {
            writeln!(out, "# Reset (hard: {}): {}/{}", hard, remote, branch)?;
        }

        let mut args = vec!["reset"];
        if hard {
            args.push("--hard");
        }
        let str = format!("{}/{}", remote, branch);
        args.push(str.as_str());
        self.git(&args, out)?;

        self.git(&["rebase", local_branch], out)
    }

    pub(crate) fn fetch_pull_request(
        &self,
        remote: &str,
        id: &usize,
        out: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        if self.verbose {
            writeln!(out, "# Fetch PR #{}", id)?;
        }

        let result = self.git(&["branch", "-D", format!("pr/{}", id).as_str()], out);
        if result.is_err() {
            // Ignore error
        }

        self.git(
            &[
                "fetch",
                remote,
                format!("refs/pull/{}/head:pr/{}", id, id).as_str(),
            ],
            out,
        )?;

        Ok(())
    }

    pub(crate) fn add_pull_request(
        &self,
        tmp_branch: &str,
        local_branch: &str,
        id: &usize,
        out: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        if self.verbose {
            writeln!(out, "# Add PR #{}", id)?;
        }

        let result = self.git(&["branch", "-D", "temp"], out);
        if result.is_err() {
            // Ignore error
        }
        self.git(&["checkout", "-b", tmp_branch], out)?;

        self.git(&["reset", "--hard", format!("pr/{}", id).as_str()], out)?;
        self.git(&["rebase", local_branch], out)?;

        self.git(&["reset", local_branch], out)?;
        self.git(&["add", "."], out)?;

        // We don't add the "#" before the PR number to avoid spamming the PR thread
        self.git(&["commit", "-m", format!("PR {}", id).as_str()], out)?;

        self.git(&["checkout", local_branch], out)?;
        self.git(&["reset", "--hard", tmp_branch], out)?;

        self.git(&["branch", "-D", tmp_branch], out)?;

        Ok(())
    }

    fn git(&self, args: &[&str], out: &mut StdoutLock) -> anyhow::Result<()> {
        exec::command("git", args, &self.work_dir, self.dry_run, out)
    }
}
