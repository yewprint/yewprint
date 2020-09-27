use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

fn main() {
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
                    .join(env!("CARGO_PKG_NAME"))
                    .join(&path)
                    .with_file_name("mod.rs.html");
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
