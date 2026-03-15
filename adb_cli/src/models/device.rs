use clap::Parser;

#[derive(Parser, Debug)]
pub enum DeviceCommands {
    /// Run a command in a shell on the device
    Shell {
        #[arg(required = true, trailing_var_arg = true)]
        commands: Vec<String>,
    },
    /// Restart adb daemon with root permissions
    Root,
}
