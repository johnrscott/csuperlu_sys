#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod sp_ienv;

pub use sp_ienv::{TuningParams, get_tuning_params, set_tuning_params};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
