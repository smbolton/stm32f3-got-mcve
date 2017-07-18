extern crate gcc;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    gcc::Config::new()
        .file("src/main.c")
        .file("src/data.c")
        // gcc-crate defaults to PIC, which results in a .got (global offset
        // table) section that doesn't get relocated properly. Turn off PIC to fix.
        // See https://github.com/japaric/cortex-m-rt/issues/22
// ---- leave .pic() true to demonstrate the bug:
        //.pic(false)
        .compile("libgot.a");

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
