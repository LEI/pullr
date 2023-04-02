use std::{io::StdoutLock, path::PathBuf};

use anyhow::Context;
use gix::{Kind, Repository, Worktree};

use crate::exec;

#[derive(Debug)]
pub(crate) struct Repo {
    dry_run: bool,
    repo: Repository,
    work_dir: PathBuf,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Commit {
    hash: String,
    message: String,
}

impl Repo {
    pub(crate) fn discover(
        path: &PathBuf,
        dry_run: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let repo = gix::discover(path)
            .with_context(|| format!("Failed to discover: {}", path.display()))?;
        let kind = repo.kind();
        let kind_worktree = Kind::WorkTree { is_linked: false };

        if kind != kind_worktree {
            return Err(format!("Invalid repo kind: {:?}", kind).into());
        }

        // Check if the repository is modified
        // match repo.kind() {
        //     Kind::Worktree({ is_linked = false }) => return Err("nope"),
        // }

        // let state = repo.state();
        // println!("State: {:#?}", state);

        // Get the current commit ID
        // let head = repo.head()?.peeled()?;
        // let current_commit_id = head.id();

        // let reference = repo.find_reference(head.name())?;
        // println!("Reference: {:#?}", reference);

        // // head_ref?
        // let head = if let Some(full_name) = repo.head_name()? {
        //     full_name.to_string()
        // } else {
        //     "nope".to_string()
        // };
        // let head_id = repo.head_id()?.to_string();

        // let index = repo.index()?;
        // println!("Index: {:#?}", index);

        // let open_index = repo.open_index()?;
        // println!("Open index: {:#?}", open_index);

        // if let Some(branch) = repo.branch_remote_ref(?) {
        //     println!("Branch: {:?}", branch?);
        // }

        let work_dir = repo.work_dir().expect("no working directory").to_path_buf();
        log::warn!("Working directory {:?}", work_dir.display());

        let repo = Self {
            dry_run,
            repo,
            work_dir,
        };

        repo.check_worktree()?;

        Ok(repo)
    }

    // pub(crate) fn fetch(&self, remote: &str, dry_run: bool) -> anyhow::Result<Status> {
    //     log::debug!("Fetching remote {:?}", remote);

    //     let name = remote;
    //     let remote = self
    //         .repo
    //         .find_remote(name)
    //         .with_context(|| "Failed to find remote")?;
    //     // let fetch_remote = match self
    //     //     .repo
    //     //     .head()
    //     //     .with_context(|| "head")?
    //     //     .into_remote(gix::remote::Direction::Fetch)
    //     // {
    //     //     Some(fetch_remote) => fetch_remote?,
    //     //     None => return Err(anyhow::Error::msg("Failed to get head fetch remote")),
    //     // };

    //     let direction = gix::remote::Direction::Fetch;
    //     // let fetch_remote = self.default_remote()?;
    //     // let name = remote.name().expect("default remote is always named");
    //     // println!("default remote -> {}", name.as_bstr());

    //     let progress = gix::progress::Discard;
    //     let should_interrupt = &gix::interrupt::IS_INTERRUPTED;
    //     let res = remote
    //         .connect(direction, progress)?
    //         .prepare_fetch(Default::default())?
    //         .with_dry_run(dry_run)
    //         .receive(should_interrupt)?;
    //     // let changes = fetch_remote
    //     //     .to_connection_with_transport(transport, gix::progress::Discard)
    //     //     .connect(Fetch, gix::progress::Discard)?
    //     //     .prepare_fetch(Default::default())?
    //     //     // .with_shallow(fetch::Shallow::Deepen(0))
    //     //     .receive(&AtomicBool::default())?;

    //     // let ref_specs = fetch_remote.refspecs(gix::remote::Direction::Fetch);
    //     let status = match &res.status {
    //         Status::NoPackReceived { update_refs } => {
    //             // print_updates(&repo, update_refs, ref_specs, res.ref_map, &mut out, err)
    //             // println!("{:#?}", update_refs);

    //             "No pack received"
    //         }
    //         Status::DryRun { update_refs } => {
    //             // print_updates(&repo, update_refs, ref_specs, res.ref_map, &mut out, err)
    //             // println!("{:#?}", update_refs);

    //             "Dry-run"
    //         }
    //         Status::Change {
    //             update_refs,
    //             write_pack_bundle,
    //         } => {
    //             // print_updates(&repo, update_refs, ref_specs, res.ref_map, &mut out, err)?;
    //             // if let Some(data_path) = write_pack_bundle.data_path {
    //             //     writeln!(out, "pack  file: \"{}\"", data_path.display()).ok();
    //             // }
    //             // if let Some(index_path) = write_pack_bundle.index_path {
    //             //     writeln!(out, "index file: \"{}\"", index_path.display()).ok();
    //             // }

    //             "Change"
    //         }
    //     };

    //     log::info!("Fetch remote status={:?}", status);

    //     // if dry_run {
    //     //     writeln!(out, "DRY-RUN: No ref was updated and no pack was received.").ok();
    //     // }

    //     Ok(res.status)
    // }

    pub(crate) fn fetch(&self, remote: &str, out: &mut StdoutLock) -> anyhow::Result<()> {
        log::debug!("Fetching remote {:?}", remote);

        exec::command(
            "git",
            vec!["fetch", remote],
            &self.work_dir,
            self.dry_run,
            out,
        )
    }

    pub(crate) fn checkout(
        &self,
        branch: &str,
        create: bool,
        out: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        log::debug!("Checking out branch {:?} (create: {})", branch, create);

        // let remote = self.default_remote()?;
        // let direction = gix::remote::Direction::Fetch;
        // let url = remote.url(direction).expect("default remote url");
        // let name = remote.name().expect("default remote is always named");
        // println!("default remote {} -> {}", name.as_bstr(), url.to_bstring());
        // let mut prepare = gix::clone::PrepareFetch::new(
        //     url.to_owned(),
        //     self.repo.work_dir().unwrap(),
        //     gix::create::Kind::WithWorktree,
        //     Default::default(),
        //     gix::open::Options::default(), // restricted(),
        // )?;
        // let (mut checkout, _out) = prepare.fetch_then_checkout(
        //     gix::progress::Discard,
        //     &std::sync::atomic::AtomicBool::default(),
        // )?;
        // let (repo, _) = checkout.main_worktree(
        //     gix::progress::Discard,
        //     &std::sync::atomic::AtomicBool::default(),
        // )?;

        // let index = repo.index()?;
        // assert_eq!(
        //     index.entries().len(),
        //     1,
        //     "All entries are known as per HEAD tree"
        // );

        // let work_dir = repo.work_dir().expect("non-bare");
        // for entry in index.entries() {
        //     let entry_path = work_dir.join(gix::path::from_bstr(entry.path(&index)));
        //     assert!(entry_path.is_file(), "{:?} not found on disk", entry_path)
        // }

        // // let git_dir = self.repo.git_dir();
        // // let mut index = gix::index::File::at(
        // //     git_dir.join("index"),
        // //     gix::hash::Kind::Sha1,
        // //     Default::default(),
        // // )?;
        // // let tree = self.tree(branch);
        // // let odb = gix::odb::at(git_dir.join("objects"))?
        // //     .into_inner()
        // //     .into_arc()?;
        // let opts = gix::worktree::index::checkout::Options::default();
        // gix::worktree::index::checkout(
        //     index,
        //     work_dir,
        //     move |oid, buf| {
        //         // if allow_return_object(oid) {
        //         //     odb.find_blob(oid, buf)
        //         // } else {
        //         Err(gix::odb::find::existing_object::Error::NotFound {
        //             oid: oid.to_owned(),
        //         })
        //         // }
        //     },
        //     &mut gix::progress::Discard,
        //     &mut gix::progress::Discard,
        //     &std::sync::atomic::AtomicBool::default(),
        //     opts,
        // );

        let mut args = vec!["checkout"];
        if create {
            args.push("-b");
        }
        args.push(branch);
        exec::command("git", args, &self.work_dir, self.dry_run, out)?;

        Ok(())
    }

    pub(crate) fn delete(&self, branch: &str, out: &mut StdoutLock) -> anyhow::Result<()> {
        log::debug!("Deleting branch {:?}", branch);

        exec::command(
            "git",
            vec!["branch", "-d", branch],
            &self.work_dir,
            self.dry_run,
            out,
        )?;

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
        log::debug!("Reset (hard: {}): {}/{}", hard, remote, branch);

        // git reset --hard pr/$PR
        let mut args = vec!["reset"];
        if hard {
            args.push("--hard");
        }
        let str = format!("{}/{}", remote, branch);
        args.push(str.as_str());
        exec::command("git", args, &self.work_dir, self.dry_run, out)?;

        // git rebase batteries
        exec::command(
            "git",
            vec!["rebase", local_branch],
            &self.work_dir,
            self.dry_run,
            out,
        )
    }

    pub(crate) fn rebase(&self, abort: bool, out: &mut StdoutLock) -> anyhow::Result<()> {
        log::debug!("Rebase (abort: {})", abort);

        let mut args = vec!["rebase"];
        if abort {
            args.push("--abort");
        }
        exec::command("git", args, &self.work_dir, self.dry_run, out)
    }

    pub(crate) fn fetch_pull_request(
        &self,
        remote: &str,
        id: &usize,
        out: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        log::debug!("Fetch PR #{}", id);
        // git branch -D pr/$PR || true

        // git fetch $REMOTE refs/pull/$PR/head:pr/$PR
        exec::command(
            "git",
            vec![
                "fetch",
                remote,
                format!("refs/pull/{}/head:pr/{}", id, id).as_str(),
            ],
            &self.work_dir,
            self.dry_run,
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
        log::debug!("Add PR #{}", id);

        // git branch -D temp || true
        let res = exec::command(
            "git",
            vec!["branch", "-D", "temp"],
            &self.work_dir,
            self.dry_run,
            out,
        );
        if res.is_err() {
            // Ignore error
        }
        // git checkout -b temp
        exec::command(
            "git",
            vec!["checkout", "-b", tmp_branch],
            &self.work_dir,
            self.dry_run,
            out,
        )?;

        // git reset --hard pr/$PR
        exec::command(
            "git",
            vec!["reset", "--hard", format!("pr/{}", id).as_str()],
            &self.work_dir,
            self.dry_run,
            out,
        )?;
        // git rebase batteries
        exec::command(
            "git",
            vec!["rebase", local_branch],
            &self.work_dir,
            self.dry_run,
            out,
        )?;

        // git reset batteries
        exec::command(
            "git",
            vec!["reset", local_branch],
            &self.work_dir,
            self.dry_run,
            out,
        )?;
        // git add .
        exec::command("git", vec!["add", "."], &self.work_dir, self.dry_run, out)?;

        // # We don't add the "#" before the PR number to avoid spamming the PR thread
        // git commit -m "PR $PR"
        exec::command(
            "git",
            vec!["commit", "-m", format!("PR {}", id).as_str()],
            &self.work_dir,
            self.dry_run,
            out,
        )?;

        // git checkout batteries
        exec::command(
            "git",
            vec!["checkout", local_branch],
            &self.work_dir,
            self.dry_run,
            out,
        )?;
        // git reset --hard temp
        exec::command(
            "git",
            vec!["reset", "--hard", tmp_branch],
            &self.work_dir,
            self.dry_run,
            out,
        )?;

        // git branch -D temp
        exec::command(
            "git",
            vec!["branch", "-D", tmp_branch],
            &self.work_dir,
            self.dry_run,
            out,
        )?;

        Ok(())
    }

    fn check_worktree(&self) -> anyhow::Result<Worktree> {
        let worktree = match self.repo.worktree() {
            Some(worktree) => worktree,
            None => return Err(anyhow::Error::msg("Invalid worktree")),
        };

        if !worktree.is_main() {
            let base = worktree.base().display();

            return Err(anyhow::Error::msg(format!(
                "Worktree is not main: {}",
                base
            )));
        }

        if worktree.is_locked() {
            let reason = worktree.lock_reason().unwrap();

            return Err(anyhow::Error::msg(format!(
                "Worktree is locked: {}",
                reason
            )));
        }

        Ok(worktree)
    }

    // fn default_remote(&self) -> anyhow::Result<gix::Remote> {
    //     let direction = gix::remote::Direction::Fetch;
    //     let remote = self
    //         .repo
    //         .find_default_remote(direction)
    //         .expect("fetch default remote")?;

    //     Ok(remote)
    // }

    // fn tree(&self, spec: &str) -> anyhow::Result<gix::Tree> {
    //     let id = self.repo.rev_parse_single(spec)?;
    //     let object = id.object()?;
    //     let peeled = object.peel_to_kind(gix::object::Kind::Tree)?;

    //     Ok(peeled.into_tree())
    // }
}
