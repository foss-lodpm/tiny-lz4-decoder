use std::{env, path::Path, process};

fn main() {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=docs.rs build detected. Process will safely exit.");
        process::exit(0);
    }

    println!("cargo:rustc-link-lib=dylib=tiny_lz4_decoder_sys");

    let home_path = env::var("HOME").expect("HOME environment variable is not set.");
    let target_dir = Path::new(&home_path).join(".local/share/tiny_lz4_decoder_sys");
    println!("cargo:rustc-link-search={}", target_dir.display());
}
