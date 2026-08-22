mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, LightCommand, ScreenCommand};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Screen { command } => match command {
            ScreenCommand::Enable => commands::screen::enable(),
            ScreenCommand::Disable => commands::screen::disable(),
            ScreenCommand::Toggle => commands::screen::toggle(),
            ScreenCommand::Status => commands::screen::status(),
        },

        Command::Light { command } => match command {
            LightCommand::Status => commands::light::status(),
            LightCommand::Set { value } => commands::light::set(&value),
        },
    }
}
