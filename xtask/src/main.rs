mod icons;

use std::{
    fs,
    path::{Path, PathBuf},
    process,
};
use xtask_wasm::{
    anyhow::{Context, Result},
    clap,
};

#[derive(clap::Parser)]
enum Cli {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
    /// Update Blueprint CSS and docs-theme CSS.
    UpdateCss,
    /// Update Blueprint icons.
    UpdateIcons,
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_module("xtask", log::LevelFilter::Info)
        .filter_module("yewprint", log::LevelFilter::Info)
        .init();

    let cli: Cli = clap::Parser::parse();

    match cli {
        Cli::Dist(dist) => {
            log::info!("Generating package...");

            let dist_dir = dist
                .static_dir_path("yewprint-doc/static")
                .run("yewprint-doc")?;

            fs::copy("yewprint-doc/src/logo.svg", dist_dir.join("favicon.svg"))?;
        }
        Cli::Watch(watch) => {
            let mut command = process::Command::new("cargo");
            command.args(["xtask", "dist"]);

            watch.run(command)?;
        }
        Cli::Start(dev_server) => {
            dev_server
                .arg("dist")
                .not_found("index.html")
                .start(xtask_wasm::default_dist_dir(false))?;
        }
        Cli::UpdateCss => download_css(true)?,
        Cli::UpdateIcons => icons::generate_icons()?,
    }

    Ok(())
}

fn download_css(force: bool) -> Result<()> {
    let static_path = PathBuf::from("yewprint-doc/static");
    let css_path = PathBuf::from("yewprint-css/src/blueprint.css");

    if force || !css_path.exists() {
        yewprint_css::download_css(&css_path)?;

        let version = yewprint_css::download_from_npm_package(
            "@blueprintjs/docs-theme",
            "3.11.1",
            Path::new("package/lib/css/docs-theme.css"),
            static_path.join("docs-theme.css"),
        )
        .context("while downloading CSS of @blueprintjs/docs-theme")?;
        log::info!("Docs Theme CSS updated to: {}", version);
    }

    Ok(())
}
