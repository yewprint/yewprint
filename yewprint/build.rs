use heck::CamelCase;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const ICON_URL: &str = "https://registry.npmjs.org/@blueprintjs/icons/-/icons-3.19.0.tgz";
const ICON_PATH: &str = "package/lib/esnext/generated/iconSvgPaths.js";

fn main() {
    let icon_svg_paths = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "curl -sSLo - {} | tar xOzf - {}",
            ICON_URL, ICON_PATH
        ))
        .output()
        .map(|x| String::from_utf8(x.stdout).expect("source file is not UTF-8"))
        .expect("could not download icons");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icon_svg_paths.rs");

    let mut src = String::new();
    let re_map = Regex::new(r"export const IconSvgPaths([0-9]+) = \{([^}]+)\};").unwrap();
    let re_item = Regex::new(r#"(?m)(?s)"([^"]+)": (\[.*?\]),$"#).unwrap();
    let mut keys = HashSet::new();

    for map in re_map.captures_iter(icon_svg_paths.as_str()) {
        src.push_str("fn icon_svg_paths_");
        src.push_str(&map[1]);
        src.push_str("(icon: IconName) -> &'static [&'static str] { match icon {\n");
        for item in re_item.captures_iter(&map[2]) {
            let key = item[1].to_camel_case();
            src.push_str("IconName::");
            src.push_str(&key);
            src.push_str(" => &");
            src.push_str(&item[2]);
            src.push_str(",\n");
            keys.insert(key);
        }
        src.push_str(" }}\n\n");
    }

    if keys.is_empty() {
        panic!("failed parse downloaded icons");
    }

    let mut keys: Vec<_> = keys.iter().collect();
    keys.sort();
    src.push_str("#[derive(Debug, Copy, Clone, PartialEq)]\npub enum IconName {\n");
    for icon in keys {
        src.push_str(icon);
        src.push_str(",\n");
    }
    src.push_str("}\n\n");

    fs::write(&dest_path, src).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
