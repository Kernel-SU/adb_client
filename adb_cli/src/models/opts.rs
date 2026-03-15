use clap::Parser;

use crate::utils;

use super::TcpCommand;

#[derive(Debug, Parser)]
#[clap(about, long_version = utils::long_version(), author)]
pub struct Opts {
    #[clap(long = "debug")]
    pub debug: bool,
    #[clap(subcommand)]
    pub command: MainCommand,
}

#[derive(Debug, Parser)]
pub enum MainCommand {
    /// TCP device related commands
    Tcp(TcpCommand),
    /// Display various version information
    Version,
}
