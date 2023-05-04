mod joi;

use thiserror::Error;

use crate::joi::{JoiDescribe, Tokenizer};

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error(transparent)]
    ParseError(#[from] serde_json::Error),

    #[error(transparent)]
    FormatError(#[from] std::fmt::Error),
}

pub fn gen_from_file(contents: String) {
    let joi_str: JoiDescribe = serde_json::from_str(contents.as_str()).expect("Something");
    joi_str.to_tokens();
}

pub fn gen(describe: String) -> Result<String, CodeGenError> {
    let joi_str: JoiDescribe = serde_json::from_str(describe.as_str())?;

    Ok(joi_str.to_tokens().to_string()?)
}
