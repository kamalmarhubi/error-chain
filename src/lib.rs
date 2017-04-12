#![allow(dead_code, unused_variables, unused_imports)] // TODO remove when stable

extern crate chrono;
extern crate crypto;
extern crate itoa;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate url;

mod cjson;
mod metadata;
mod error;
mod tuf;

pub use tuf::*;
pub use error::*;
