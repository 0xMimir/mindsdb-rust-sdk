#[macro_use]
extern crate serde;

mod models;

mod error;
pub use error::{Error, Result};

mod sdk;
pub use sdk::MindsDb;
