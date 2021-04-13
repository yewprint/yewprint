#![allow(clippy::unnecessary_wraps)]

use anyhow::Result;
use std::process::Command;
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main("yewprint-doc", pre_build)]
#[derive(StructOpt, Debug)]
enum Cli {}

fn pre_build(_args: &DefaultBuildArgs, profile: BuildProfile, cargo: &mut Command) -> Result<()> {
    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            cargo.args(&["--features", "wee_alloc"]);
        }
        BuildProfile::Dev => {
            cargo.args(&["--features", "console_error_panic_hook"]);
        }
    }

    Ok(())
}
