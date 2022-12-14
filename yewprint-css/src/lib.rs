use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::path::Path;

const LATEST_BLUEPRINT_WORKING_VERSION: &str = "3.54.0";

pub const BLUEPRINT_CSS: &str = include_str!("blueprint.css");

/// Download the CSS of Blueprint to a provided destination path.
pub fn download_css(dest: impl AsRef<Path>) -> Result<()> {
    let version = download_from_npm_package(
        "@blueprintjs/core",
        LATEST_BLUEPRINT_WORKING_VERSION,
        Path::new("package/lib/css/blueprint.css"),
        dest,
    )
    .context("while downloading CSS of @blueprintjs/core")?;
    log::info!("Blueprint CSS updated to: {}", version);
    Ok(())
}

/// Download any file from NPM package's latest version.
pub fn download_from_npm_package(
    package_name: &str,
    version: &str,
    src: impl AsRef<Path>,
    dest: impl AsRef<Path>,
) -> Result<String> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    let version = if version.is_empty() || version == "latest" {
        let info: PackageInfo =
            ureq::get(format!("https://registry.npmjs.org/{}", package_name).as_str())
                .call()?
                .into_json()?;

        info.dist_tags.latest
    } else {
        version.to_string()
    };

    let resp = ureq::get(
        format!(
            "https://registry.npmjs.org/{}/-/{}-{}.tgz",
            package_name,
            package_name
                .split_once('/')
                .map(|(_, name)| name)
                .unwrap_or(package_name),
            version,
        )
        .as_str(),
    )
    .call()?;

    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(resp.into_reader()));

    let blueprint_css = archive.entries()?.find(|entry| {
        entry
            .as_ref()
            .ok()
            .and_then(|entry| entry.path().map(|path| path == src).ok())
            .unwrap_or(false)
    });

    if let Some(entry) = blueprint_css {
        let mut entry = entry.unwrap();
        entry.unpack(dest)?;
        Ok(version)
    } else {
        bail!("could not find `{}` in archive!", src.display());
    }
}

#[derive(Deserialize)]
struct PackageInfo {
    #[serde(rename = "dist-tags")]
    dist_tags: PackageDistTags,
}

#[derive(Deserialize)]
struct PackageDistTags {
    latest: String,
}
