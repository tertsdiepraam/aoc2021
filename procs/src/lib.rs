use proc_macro::TokenStream;
use std::fs;

#[proc_macro]
pub fn mods(_item: TokenStream) -> TokenStream {
    let paths = fs::read_dir("./src").unwrap();

    let mut out = String::new();
    for path in paths {
        let name = path.unwrap().file_name();
        let name = name.to_string_lossy();
        if name != "main.rs"  && !name.starts_with("_") {
            out += &format!("mod {};", name.split_once('.').unwrap().0);
        }
    }
    out.parse().unwrap()
}

#[proc_macro]
pub fn bench_all(_item: TokenStream) -> TokenStream {
    let mut paths: Vec<_> = fs::read_dir("./src").unwrap().collect();
    paths.sort_by_key(|p| p.as_ref().unwrap().file_name());

    let mut out: String = "[".into();
    for path in paths {
        let name = path.unwrap().file_name();
        let name = name.to_string_lossy();
        let name = &name[0..name.len()-3];
        if name != "main" && !name.starts_with("_") {
            let day: u8 = name[3..].parse().unwrap();
            out += &format!("bench({0}, &exercises, day{0}::main),", day);
        }
    }
    out += "].into_iter().flat_map(|x| x).sum::<Duration>()";
    out.parse().unwrap()
}