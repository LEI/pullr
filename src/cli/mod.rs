use clap::Parser;

/// Pullr
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Remote repository.
    #[arg(short, long, default_value = "origin")]
    remote: String,

    /// Main branch.
    #[arg(short, long, default_value = "master")]
    branch: String,

    // #[arg(short, long = "pull")]
    pull_requests: Vec<String>,
}

// https://github.com/Byron/gitoxide/blob/main/gix/examples/stats.rs
fn detect_current_repo() -> Result<(), Box<dyn std::error::Error>> {
    let mut repo = gix::discover(".")?;
    println!(
        "Repo: {}",
        repo.work_dir().unwrap_or_else(|| repo.git_dir()).display()
    );

    let mut max_commit_size = 0;
    let mut avg_commit_size = 0;
    repo.object_cache_size(32 * 1024);
    let commit_ids = repo
        .head()?
        .into_fully_peeled_id()
        .ok_or("There are no commits - nothing to do here.")??
        .ancestors()
        .all()?
        .inspect(|id| {
            if let Ok(Ok(object)) = id.as_ref().map(|id| id.object()) {
                avg_commit_size += object.data.len();
                if object.data.len() > max_commit_size {
                    max_commit_size = object.data.len();
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    println!("Num Commits: {}", commit_ids.len());
    println!("Max commit Size: {}", max_commit_size);
    println!("Avg commit Size: {}", avg_commit_size / commit_ids.len());

    Ok(())
}

pub fn parse() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{:#?}", args);

    detect_current_repo()
}
