use anyhow::Result;
use std::process::Command;
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main("yewprint-doc", pre_build = pre_build)]
#[derive(StructOpt, Debug)]
enum Cli {}

fn pre_build(_args: &DefaultBuildArgs, profile: BuildProfile, command: &mut Command) -> Result<()> {
    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            command.args(&["--features", "wee_alloc"]);
        }
        BuildProfile::Dev => {
            command.args(&["--features", "console_error_panic_hook"]);
        }
    }

    Ok(())
}
