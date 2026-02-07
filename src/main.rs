use anyhow::{Context, Result};
use clap::Parser;
use clio::*;
use mlua::Lua;
use std::env;
use std::fs;

const MANIFEST_FILENAME: &str = "loxp.lua";

/// Lua OXidized Packages
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Override current working directory
    #[arg(long)]
    workspace: Option<ClioPath>,
    /// Function name to run in the manifest
    function: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let workspace = match cli.workspace {
        Some(mut dir) => dir.join(MANIFEST_FILENAME),
        None => ClioPath::new(MANIFEST_FILENAME)?,
    };

    let lua = Lua::new();

    let current_dir = env::current_dir()?.to_string_lossy().replace('\\', "/");

    lua.load(&format!(
        r#"
        package.path = "{}/?.lua;" .. package.path
        "#,
        current_dir
    ))
    .exec()?;

    match cli.function {
        Option::Some(func_name) => {
            lua.load(
                fs::read_to_string(workspace.path())
                    .context(format!("Couldn't load the {MANIFEST_FILENAME} file"))?,
            )
            .exec()?;
            let run_func = lua.globals().get::<mlua::Function>(func_name)?;
            run_func.call::<Option<mlua::Number>>(mlua::Nil)?;
        }
        None => {
            lua.load(
                fs::read_to_string(workspace.path())
                    .context(format!("Couldn't load the {MANIFEST_FILENAME} file"))?,
            )
            .call::<Option<mlua::Function>>(mlua::Nil)?;
        }
    }

    Ok(())
}
