use clap::Parser;
use ffmpreg::EXIT_SUCCESS;
use ffmpreg::cli::color;
use ffmpreg::cli::{Cli, executor};

fn main() {
	let cli = Cli::parse();
	if let Err(message) = executor::execute(cli) {
		message.render_and_exit();
	}
	color::print_success(None);
	std::process::exit(EXIT_SUCCESS);
}
