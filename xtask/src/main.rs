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
    UpdateCSS,
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_module("xtask", log::LevelFilter::Info)
        .filter_module("yewprint", log::LevelFilter::Info)
        .init();

    let cli: Cli = clap::Parser::parse();

    match cli {
        Cli::Dist(arg) => {
            log::info!("Generating package...");

            download_css(false)?;

            let result = arg
                .static_dir_path("yewprint-doc/static")
                .run("yewprint-doc")?;

            fs::copy(
                "yewprint-doc/src/logo.svg",
                result.dist_dir.join("favicon.svg"),
            )?;
        }
        Cli::Watch(arg) => {
            let mut command = process::Command::new("cargo");
            command.args(["xtask", "dist"]);

            arg.run(command)?;
        }
        Cli::Start(arg) => {
            arg.arg("dist").start(xtask_wasm::default_dist_dir(false))?;
        }
        Cli::UpdateCSS => download_css(true)?,
    }

    Ok(())
}

fn download_css(force: bool) -> Result<()> {
    let static_path = PathBuf::from("yewprint-doc/static");
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
