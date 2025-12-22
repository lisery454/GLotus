// build.rs
use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use std::{env, fs::File, path::Path};

#[cfg(windows)]
fn main() {
    // Windows需要额外处理
    println!("cargo:rustc-link-lib=opengl32");
    generate_bindings();
}

#[cfg(not(windows))]
fn main() {
    generate_bindings();
}

fn generate_bindings() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set");
    let dest_path = Path::new(&out_dir).join("gl_bindings.rs");

    println!("cargo:rerun-if-changed=build.rs"); // 只有build.rs变化时重新生成

    let mut file = File::create(&dest_path).unwrap_or_else(|e| {
        panic!("Failed to create gl_bindings.rs: {}", e);
    });

    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .unwrap_or_else(|e| {
            panic!("Failed to generate OpenGL bindings: {}", e);
        });
}
