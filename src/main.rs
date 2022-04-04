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

fn pre_build(args: &DefaultBuildArgs, profile: BuildProfile, cargo: &mut Command) -> Result<()> {
    let package = args.frontend_package();

    download_css(package, false)?;

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
            download_css(package, true)?;
        }
    }

    Ok(())
}

fn download_css(package: &Package, force: bool) -> Result<()> {
    let static_path = package.manifest_path.parent().unwrap().join("static");
    let css_path = static_path.join("blueprint.css");

    if force || !css_path.exists() {
        yewprint_css::download_css(&css_path)?;

        let version = yewprint_css::download_from_npm_package(
            "@blueprintjs/docs-theme",
            "3.11.1",
            Path::new("package/lib/css/docs-theme.css"),
            &static_path.join("docs-theme.css"),
        )
        .context("while downloading CSS of @blueprintjs/docs-theme")?;
        log::info!("Docs Theme CSS updated to: {}", version);
    }

    Ok(())
}
