use clap::Parser;

use pullr::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    cli.run()
}
