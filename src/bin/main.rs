use clap::Parser;
use log::trace;

use pullr::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    trace!("{:#?}", cli);

    cli.run()
}
