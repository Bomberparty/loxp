use anyhow::{Context, Result};
use clap::Parser;
use mlua::Lua;
use std::env;
use std::fs;

const DEFAULT_MANIFEST_FILENAME: &str = "loxp.lua";

/// Lua OXidized Packages
#[derive(Parser)]
#[command(
    version,
    about,
    long_about=None,
    help_template = "\
{before-help}{about-with-newline}
{usage-heading} {usage} ...

{all-args}{after-help}\
"
)]
struct Cli {
    /// Override filename (might not work on files in directores,
    /// i.e. ./dir/your_file.lua)
    #[arg(long, default_value = DEFAULT_MANIFEST_FILENAME)]
    filename: String,
    /// Whether to pass command-line arguments to a second argument of lua function
    #[arg(long)]
    args: bool,
    /// Function name to run in the manifest
    function: Option<String>,
    #[arg(allow_hyphen_values = true, trailing_var_arg = true, hide = true, num_args = 0..)]
    rest: Vec<String>,
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

    let loxp_table: mlua::Table = lua
        .load(
            fs::read_to_string(&cli.filename)
                .context(format!("Couldn't load the '{}' file", cli.filename))?,
        )
        .into_function()?
        .call::<mlua::Table>(())?;

    let func_name = cli.function.unwrap_or(String::from("default"));

    let func: mlua::Function = loxp_table.get(func_name.as_str()).context(format!(
        "Could not load function '{}' from 'loxp' table",
        func_name
    ))?;

    func.call::<()>(loxp_table)
        .context(format!("Error executing function '{}'", func_name))?;

    Ok(())
}
