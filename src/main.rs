use mlua::Lua;
use anyhow::Result;
use clap::Parser;
use std::fs;
use clio::*;

const MANIFEST_FILENAME: &str = "manifest.loxp.lua"; 

/// Simple mlua test
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Override current working directory
    #[arg(long)]
    workdir: Option<ClioPath>,
    /// Function name to run in the manifest
    function: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let workdir = match cli.workdir {
        Some(mut dir) => dir.join(MANIFEST_FILENAME),
        None => ClioPath::new(MANIFEST_FILENAME)?
    };

    let lua = Lua::new();

    match cli.function {
        Option::Some(func_name) => {
            lua.load(fs::read_to_string(workdir.path())?).exec()?;
            let run_func = lua.globals().get::<mlua::Function>(func_name)?;
            run_func.call::<Option<mlua::Number>>(mlua::Nil)?;
        },
        None => {
            lua.load(fs::read_to_string(workdir.path())?).call::<Option<mlua::Number>>(mlua::Nil)?;
        }
    }

    Ok(())
}