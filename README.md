# LOXP - Lua OXidized Packages

A minimal task runner and package manager that uses Lua modules for task definition and execution. The project provides a single binary that embeds Lua, allowing scripts to run consistently across different operating systems without requiring a separate Lua installation.

## Purpose

Loxp is designed to offer a lightweight alternative to existing task runners and package managers by leveraging Lua's simplicity and portability. It focuses on minimal dependencies and cross-platform consistency, making it suitable for automation scripts, build tasks, and small-scale package management.

## Current Features

- Single binary with embedded Lua (via mlua)
- Task definition via Lua modules
- CLI interface for executing specific manifests
- Basic package.path configuration for module resolution from your current working directory

## Future Plans

- Support for running tasks directly from single-file archives, like your favorite package manager does
- Interactive mode for task exploration and execution

## Usage

1. Define tasks in a Lua module (default: `loxp.lua`):
   ```lua
   local m = {}
   m.PACKAGE_VERSION = "1.0"

   function m:build()
       print("Building...")
   end

   function m:default()
       print("Running default task")
   end

   return m
   ```

2. Run tasks via the Loxp binary:
   ```bash
   # Run the default task
   loxp

   # Run a specific function
   loxp build
   ```

3. Override the default filename if needed:
   ```bash
   loxp --filename custom.lua test
   ```

## Contributing

Suggestions and issues are welcome via the project's issue tracker. Please provide clear descriptions and context for any proposed changes or reported problems.

## Technical Details

- Built in Rust with mlua for Lua 5.4 embedding
- CLI powered by clap
- Error handling via anyhow

## Notes

This is an early-stage project. Behavior and APIs may evolve as development continues, though for the version 1.x I will try my best to not to break the current interface.