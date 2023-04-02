use clap::Parser;

use pullr::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    cli.run()
}
