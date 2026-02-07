use anyhow::{Context, Result};
use clap::Parser;
use clio::*;
use mlua::Lua;
use std::env;
use std::fs;

const DEFAULT_MANIFEST_FILENAME: &str = "loxp.lua";

/// Lua OXidized Packages
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Override filename
    #[arg(long, default_value = DEFAULT_MANIFEST_FILENAME)]
    filename: String,
    /// Function name to run in the manifest
    function: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let lua = Lua::new();

    let current_dir = env::current_dir()?.to_string_lossy().replace('\\', "/");
    lua.load(&format!(
        "package.path = \"{}/?.lua;\" .. package.path",
        current_dir
    ))
        .exec()?;

    let lua_code = fs::read_to_string(&cli.filename)
        .context(format!("Couldn't load the '{}' file", cli.filename))?;

    match cli.function {
        Some(func_name) => {
            lua.load(&lua_code).exec()?;
            let globals = lua.globals();
            let run_func: mlua::Function = globals.get::<mlua::Function>(func_name)?;
            run_func.call::<()>(())?;
        }
        None => {
            lua.load(&lua_code).exec()?;
        }
    }

    Ok(())
}
