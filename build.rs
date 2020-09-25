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

    #[cfg(feature = "doc")]
    load_examples_with_colors();
}

#[cfg(feature = "doc")]
fn load_examples_with_colors() {
    use std::ffi::OsString;
    use syntect::highlighting::{Theme, ThemeSet};
    use syntect::parsing::SyntaxSet;

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    fn recursive<P: AsRef<Path>>(
        base_path: P,
        syntax_set: &SyntaxSet,
        theme: &Theme,
        out_dir: &OsString,
    ) {
        for entry in fs::read_dir(base_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                recursive(&path, syntax_set, theme, out_dir);
            }

            let file_name = path.file_name().unwrap().to_str().unwrap();

            if path.starts_with("./src") && file_name == "example.rs" && path.is_file() {
                let dest_path = Path::new(&out_dir)
                    .join(&path)
                    .with_file_name("doc.rs.html");
                let src =
                    syntect::html::highlighted_html_for_file(&path, syntax_set, theme).unwrap();

                let _ = std::fs::create_dir_all(dest_path.parent().unwrap());
                fs::write(&dest_path, src.trim_end()).unwrap();
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }

    recursive(
        ".",
        &syntax_set,
        &theme_set.themes["Solarized (dark)"],
        &out_dir,
    )
}
