use std::env::var;

fn main() {
    let ecos_link_type = env_or_default("CARGO_ecos_TYPE", "dylib");
    let ecos_link_name = env_or_default("CARGO_ecos", "ecos");

    println!("cargo:rustc-link-lib={}={}", ecos_link_type, ecos_link_name);
}

fn env_or_default(var_name: &str, default: &str) -> String {
    match var(var_name) {
        Ok(s) => s,
        Err(_) => default.into(),
    }
}
