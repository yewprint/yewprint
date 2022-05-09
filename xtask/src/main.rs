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
    Dist(Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

#[derive(clap::Parser)]
struct Dist {
    /// Force CSS update
    #[clap(long, short = 'f')]
    force: bool,

    #[clap(flatten)]
    base: xtask_wasm::Dist,
}

fn main() -> Result<()> {
    let cli: Cli = clap::Parser::parse();

    env_logger::builder()
        .filter(Some("xtask"), log::LevelFilter::Info)
        .init();

    match cli {
        Cli::Dist(arg) => {
            log::info!("Generating package...");

            let result = arg
                .base
                .static_dir_path("yewprint-doc/static")
                .run("yewprint-doc")?;

            download_css(&result.dist_dir, arg.force)?;

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
    }

    Ok(())
}

fn download_css(dest: impl AsRef<Path>, force: bool) -> Result<()> {
    let static_path = PathBuf::from("yewprint-doc/static");
    let css_path = static_path.join("blueprint.css");

    if force || !css_path.exists() {
        log::info!("Downloading Blueprint CSS");
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

    fs::copy(css_path, dest.as_ref().join("blueprint.css"))?;

    Ok(())
}
