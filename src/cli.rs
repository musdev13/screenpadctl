use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "screenpadctl", version, about = "Control the ASUS ScreenPad")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Control ScreenPad power
    Screen {
        #[command(subcommand)]
        command: ScreenCommand,
    },

    /// Control ScreenPad brightness
    Light {
        #[command(subcommand)]
        command: LightCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum ScreenCommand {
    /// Enable ScreenPad
    Enable,

    /// Disable ScreenPad
    Disable,

    /// Toggle ScreenPad
    Toggle,

    /// Show ScreenPad status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum LightCommand {
    /// Show brightness
    Status,

    /// Set brightness
    ///
    /// Examples:
    ///   100
    ///   +20
    ///   -20
    Set {
        /// Absolute value or relative change
        value: String,
    },
}
