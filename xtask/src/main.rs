use std::{fs, path::Path, process};
use xtask_wasm::{anyhow::Result, clap, metadata};

#[derive(clap::Parser)]
enum Cli {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
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
                .static_dir_path("yewprint-doc/static")
                .run("yewprint-doc")?;

            download_css(result.dist_dir)?;
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

fn download_css(dest: impl AsRef<Path>) -> Result<()> {
    let css_path = metadata().target_directory.join("blueprint.css");

    if !css_path.exists() {
        log::info!("Downloading Blueprint CSS");
        yewprint_css::download_css(&css_path)?;
    }

    fs::copy(css_path, dest.as_ref().join("blueprint.css"))?;

    Ok(())
}
