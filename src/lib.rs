use anyhow::Result;

mod convert;
pub mod cli;

pub fn convert(input: &str) -> Result<String> {
    Ok(convert::convert_markdown(input))
}
