use anyhow::{Context, Result};
use clap::Parser;
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

    let _ = lua
        .load(
            fs::read_to_string(&cli.filename)
                .context(format!("Couldn't load the '{}' file", cli.filename))?,
        )
        .exec()?;
    let loxp_table = lua.globals().get::<mlua::Table>("loxp")?;

    match cli.function {
        Some(func_name) => {
            let func: mlua::Function = loxp_table
                .get(func_name.as_str())
                .context(format!("Could not load function '{}' from 'loxp' table", func_name))?;
            func.call::<()>(())
                .context(format!("Error executing function '{}'", func_name))?;
        }
        None => {
            let default_func: mlua::Function = loxp_table
                .get("default")
                .context("Could not load the default function from 'loxp' table".to_string())?;
            default_func.call::<()>(())
                .context("Error executing the default function".to_string())?;
        }
    }

    Ok(())
}
