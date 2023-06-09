mod joi;
mod joi_types;
use thiserror::Error;

use crate::joi::JoiDescribe;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),
}

pub fn gen(describe: String) -> Result<String, CodeGenError> {
    let joi_str: JoiDescribe = serde_json::from_str((describe).as_str())?;

    Ok(joi_str.convert()?)
}
