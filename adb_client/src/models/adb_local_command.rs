use std::fmt::Display;

/// ADB commands that relates to an actual device.
pub enum ADBLocalCommand {
    /// Execute a shell command with optional arguments
    ShellCommand(String, Vec<String>),
    /// Restart adb daemon with root permissions
    Root,
}

impl Display for ADBLocalCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShellCommand(command, shell_args) => {
                let args_s = shell_args.join(",");
                write!(
                    f,
                    "shell{}{args_s},raw:{command}",
                    if shell_args.is_empty() { "" } else { "," }
                )
            }
            Self::Root => write!(f, "root:"),
        }
    }
}
