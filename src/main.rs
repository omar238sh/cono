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
            let lua = create_lua_instance().map_err(|e| {
                RenderError::CustomError(format!("failed to create lua instance {e}"))
            })?;
            let script = read_to_string(file_path.clone())
                .map_err(|_| RenderError::CustomError("Failed to find file".to_string()))?;
            lua.load(script)
                .set_name(file_path.to_str().unwrap())
                .exec()
                .map_err(|e| RenderError::CustomError(e.to_string()))?;
        }
        Commands::Init => init()?,
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
    Init,
}

pub fn init() -> Result<(), RenderError> {
    let cwd = std::env::current_dir()?;
    let init_path = cwd.join("init.lua");
    let luarc_path = cwd.join(".luarc.json");

    let cono_lib_dir = dirs::data_dir()
        .ok_or_else(|| RenderError::CustomError("cannot resolve data dir".to_string()))?
        .join("cono");
    let init_content = r##"local compose = require("compose")

local scene = compose.scene(Size.youtube_shorts(), FpsTarget.f60())

local title = compose.text("Golden Ratio", 56, "#F5E0DC", Weight.bold())
local eq = compose.math("phi = (1 + sqrt(5)) / 2", 48)

scene
    :actor("t", title, compose.center())
    :actor("e", eq, compose.center())
    :enter("t", compose.fade_in(1.0))
    :hold(2.5)
    :exit("t", compose.fade_out(0.8))
    :enter("e", compose.spin_in(1.2))
    :hold(3.0)
    :pulse("e", compose.pulse(0.4, 5.0))
    :exit("e", compose.slide_out("down", 0.8))

scene:render("output.mp4")"##;
    let cono_lib = cono_lib_dir.display().to_string();

    let luarc_content = format!(
        r#"{{
          "workspace.library": [
            "src/render/lua/lua_definitions.lua",
            "{cono_lib}"
          ],
          "diagnostics.globals": [
            "model",
            "Easing",
            "require"
          ]
        }}
        "#
    );

    std::fs::write(&init_path, init_content)?;
    std::fs::write(&luarc_path, luarc_content)?;

    println!("wrote {} and {}", init_path.display(), luarc_path.display());
    Ok(())
}
