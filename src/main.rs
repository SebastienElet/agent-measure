mod cli;
mod cli_output;
mod measure_cli;
use clap::Parser;
fn main() -> std::process::ExitCode {
    measure_cli::run_measure(cli::Cli::parse().command)
}
