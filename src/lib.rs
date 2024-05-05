#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate log;
pub use log::*;
extern crate pretty_env_logger;
pub use pretty_env_logger::env_logger::builder as pretty_env_logger_builder;
pub use pretty_env_logger::init as init_pretty_env_logger;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

#[macro_use]
pub mod _macro;
pub use _macro::*;

pub mod bound_method;
pub mod chunk;
pub mod class;
pub mod class_compiler;
pub mod closure;
pub mod compiler;
pub mod constants;
pub mod error;
pub mod function;
pub mod garbage_collection;
pub mod instance;
pub mod instruction;
pub mod instructions;
pub mod interpreter;
pub mod local;
pub mod native_function;
pub mod parser;
pub mod scanner;
pub mod standard_library;
pub mod string;
pub mod table;
pub mod token;
pub mod value;
pub mod virtual_machine;

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  #[allow(unused_imports)]
  use super::*;

  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }
}
