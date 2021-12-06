use proc_macro::TokenStream;
use std::fs;

#[proc_macro]
pub fn mods(_item: TokenStream) -> TokenStream {
    let paths = fs::read_dir("./src").unwrap();

    let mut out = String::new();
    for path in paths {
        let name = path.unwrap().file_name();
        if name != "main.rs" {
            out += &format!("mod {};", name.to_string_lossy().split_once('.').unwrap().0);
        }
    }
    out.parse().unwrap()
}

#[proc_macro]
pub fn exercises(_item: TokenStream) -> TokenStream {
    let mut paths: Vec<_> = fs::read_dir("./src").unwrap().collect();
    paths.sort_by_key(|p| p.as_ref().unwrap().file_name());

    let mut out: String = format!("const EXERCISES: [fn() -> u64; {}] = [", 2*paths.len() - 2);
    for path in paths {
        let name = path.unwrap().file_name();
        if name != "main.rs" {
            out += &format!("{0}::first, {0}::second,", name.to_string_lossy().split_once('.').unwrap().0);
        }
    }
    out += "];";
    out.parse().unwrap()
}