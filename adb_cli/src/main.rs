#![doc = include_str!("../README.md")]

mod models;
mod utils;

use adb_client::ADBDeviceExt;
use adb_client::tcp::ADBTcpDevice;

use clap::Parser;
use models::{DeviceCommands, MainCommand, Opts};
use std::process::ExitCode;
use utils::setup_logger;

use crate::models::ADBCliResult;

fn run_command(mut device: Box<dyn ADBDeviceExt>, command: DeviceCommands) -> ADBCliResult<()> {
    match command {
        DeviceCommands::Shell { commands } => {
            device.shell_command(&commands.join(" "), Some(&mut std::io::stdout()), None)?;
        }
        DeviceCommands::Root => {
            device.root()?;
            log::info!("Restarted adbd as root");
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = inner_main() {
        log::error!("{err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn inner_main() -> ADBCliResult<()> {
    let opts = Opts::parse();

    setup_logger(opts.debug);

    match opts.command {
        MainCommand::Tcp(tcp_command) => {
            let device = ADBTcpDevice::new(tcp_command.address)?;
            run_command(device.boxed(), tcp_command.commands)?;
        }
        MainCommand::Version => {
            println!("{} {}", env!("CARGO_PKG_NAME"), utils::long_version());
        }
    }

    Ok(())
}
