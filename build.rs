extern crate bindgen;
    
use std::collections::HashSet;
use std::sync::RwLock;
use std::{env, sync::Arc};
use std::path::PathBuf;

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};

// Added as described here: "https://github.com/rust-lang/rust-bindgen/
// issues/687#issuecomment-416537395" to handle issues with FP_NAN
// etc.
#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        if name == "FP_NAN" {
            return MacroParsingBehavior::Ignore
        } else if name == "FP_INFINITE" {
            return MacroParsingBehavior::Ignore
        } else if name == "FP_ZERO" {
            return MacroParsingBehavior::Ignore
        } else if name == "FP_SUBNORMAL" {
            return MacroParsingBehavior::Ignore
        } else if name == "FP_NORMAL" {
            return MacroParsingBehavior::Ignore
        }

	
        MacroParsingBehavior::Default
    }
}

fn main() {

    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    // let dst = cmake::build("superlu-5.3.0");

    // println!("cargo:rustc-link-search=native={}", dst.display());
    // println!("cargo:rustc-link-lib=static=foo");
    
    // println!("cargo:rustc-link-lib=superlu");

    let macros = Arc::new(RwLock::new(HashSet::new()));
    
    let bindings = bindgen::Builder::default()
        .header("superlu-5.3.0/SRC/slu_ddefs.h")
        .parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
