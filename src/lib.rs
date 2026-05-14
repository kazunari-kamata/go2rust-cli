pub mod cli;
pub mod error;
pub mod generator;
pub mod ir;
pub mod parser;

use error::Result;

pub fn convert_source(source: &str) -> Result<String> {
    let items = parser::parse(source)?;
    Ok(generator::generate(&items))
}
