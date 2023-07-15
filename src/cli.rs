use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
	pub file_path: String,
}
