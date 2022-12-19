use heck::ToUpperCamelCase;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use xtask_wasm::anyhow::{ensure, Context, Result};

const LATEST_BLUEPRINT_WORKING_VERSION: &str = "3.33.0";

pub(crate) fn generate_icons() -> Result<()> {
    let js_file = Path::new("iconSvgPaths.js");
    let version = yewprint_css::download_from_npm_package(
        "@blueprintjs/icons",
        LATEST_BLUEPRINT_WORKING_VERSION,
        Path::new("package/lib/esnext/generated/iconSvgPaths.js"),
        js_file,
    )
    .context("while downloading icons of @blueprintjs/icons")?;
    log::info!("Blueprint icons updated to: {}", version);

    let icon_svg_paths = fs::read_to_string(js_file).context("cannot read file")?;
    let out_dir = Path::new("src");
    let dest_path = Path::new(&out_dir).join("icon_svg_paths.rs");

    let mut src = String::new();
    let re_map = Regex::new(r"export const IconSvgPaths([0-9]+) = \{([^}]+)\};").unwrap();
    let re_item = Regex::new(r#"(?m)(?s)"([^"]+)": (\[.*?\]),\s*$"#).unwrap();
    let mut keys = HashSet::new();

    for map in re_map.captures_iter(icon_svg_paths.as_str()) {
        src.push_str("fn icon_svg_paths_");
        src.push_str(&map[1]);
        src.push_str("(icon: &Icon) -> &'static [&'static str] { match icon {\n");
        for item in re_item.captures_iter(&map[2]) {
            let key = item[1].to_upper_camel_case();
            src.push_str("Icon::");
            src.push_str(&key);
            src.push_str(" => &");
            src.push_str(&item[2]);
            src.push_str(",\n");
            keys.insert(key);
        }
        src.push_str("Icon::Custom(_) => &[],\n");
        src.push_str(" }}\n\n");
    }

    ensure!(!keys.is_empty(), "failed parse icons");

    let mut keys: Vec<_> = keys.iter().collect();
    keys.sort();
    src.push_str(
        "#[derive(Debug, Clone, PartialEq)]\n\
         pub enum Icon {\n",
    );
    for icon in &keys {
        src.push_str(icon);
        src.push_str(",\n");
    }
    src.push_str("Custom(Html),\n");
    src.push_str("}\n\n");

    src.push_str("impl Icon {\n");
    src.push_str("pub const ALL: &[Icon] = &[\n");
    for icon in keys {
        src.push_str("Icon::");
        src.push_str(icon);
        src.push_str(",\n");
    }
    src.push_str("];\n");
    src.push('}');

    fs::write(dest_path, src).context("could not write into file")?;

    Ok(())
}
