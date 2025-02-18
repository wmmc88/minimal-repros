use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    pub test_arg: Option<String>,

    #[arg(long, hide_long_help(true))] // false fixes the issue of newlines
    /// Process all packages in the workspace
    pub all: bool,
}

fn main() {
    Cli::parse();
}
