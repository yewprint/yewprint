#![allow(clippy::unnecessary_wraps)]

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main("yewprint-doc", pre_build, other_cli_commands)]
#[derive(StructOpt, Debug)]
enum Cli {
    /// Update Blueprint CSS and docs-theme CSS.
    UpdateCss,
}

fn pre_build(_args: &DefaultBuildArgs, profile: BuildProfile, cargo: &mut Command) -> Result<()> {
    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            cargo.args(&["--features", "wee_alloc"]);
        }
        BuildProfile::Dev => {
            cargo.args(&["--features", "console_error_panic_hook"]);
        }
    }
    fs::copy("yewprint-doc/src/logo.svg", "build/favicon.svg")?;

    Ok(())
}

fn other_cli_commands(cli: Cli, _metadata: &Metadata, package: &Package) -> Result<()> {
    match cli {
        Cli::UpdateCss => {
            yewprint_css::download_css(
                &package
                    .manifest_path
                    .parent()
                    .unwrap()
                    .join("static")
                    .join("blueprint.css"),
            )?;

            let version = yewprint_css::download_from_npm_package(
                "@blueprintjs/docs-theme",
                Path::new("package/lib/css/docs-theme.css"),
                &package
                    .manifest_path
                    .parent()
                    .unwrap()
                    .join("static")
                    .join("docs-theme.css"),
            )
            .context("while downloading CSS of @blueprintjs/docs-theme")?;
            println!("Docs Theme CSS updated to: {}", version);
        }
    }

    Ok(())
}
