extern crate bindgen;
    
use std::collections::HashSet;
use std::sync::RwLock;
use std::{env, sync::Arc};
use std::path::PathBuf;

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use cmake::Config;

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
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    let dst = Config::new("superlu-5.3.0")
    // Try to link with the system libblas. There is also an option to link
    // with an internally bundled BLAS, but there is a warning that this may
    // be very slow (see superlu-5.3.0/README.
        .define("TPL_BLAS_LIBRARIES", "blas")
        .build();

    // If you link to an external BLAS library, you also need to specify which
    // one here. TODO: do some research into what BLAS version is present/how to
    // give the user options.
    println!("cargo:rustc-link-lib=dylib=blas");
    
    // Is there any way this could find the wrong SuperLU?
    println!("cargo:rustc-link-search=native={}", dst.join("build/SRC").display());
    println!("cargo:rustc-link-lib=static=superlu");
    
    let macros = Arc::new(RwLock::new(HashSet::new()));
    
    let bindings = bindgen::Builder::default()
        .header("superlu-5.3.0/SRC/slu_sdefs.h")
        .header("superlu-5.3.0/SRC/slu_ddefs.h")
        .header("superlu-5.3.0/SRC/slu_cdefs.h")
        .header("superlu-5.3.0/SRC/slu_zdefs.h")
        .parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
