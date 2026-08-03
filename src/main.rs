use clap::{Parser, Subcommand};
use std::{fs::read_to_string, path::PathBuf};

use crate::render::{error::RenderError, lua::lua_test::create_lua_instance};

mod assets;
mod fonts;
mod render;

fn main() -> Result<(), RenderError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render { file_path } => {
            let lua = create_lua_instance().map_err(|_| {
                RenderError::CustomError("failed to create lua instance".to_string())
            })?;
            let script = read_to_string(file_path)
                .map_err(|_| RenderError::CustomError("Failed to find file".to_string()))?;
            lua.load(script)
                .exec()
                .map_err(|e| RenderError::CustomError(e.to_string()))?;
        }
    }
    Ok(())
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Render { file_path: PathBuf },
}
