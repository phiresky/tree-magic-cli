use built::write_built_file_with_opts;
use std::*;

fn main() {
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    write_built_file_with_opts(
        built::Options::default().set_dependencies(true),
        src.as_ref(),
        &dst,
    )
    .expect("Failed to acquire build-time information");
}
