use std::collections::BTreeMap;

use monostate::MustBe;
use serde::{self, Deserialize, Serialize};

use crate::joi::JoiDescribe;

// https://stackoverflow.com/questions/61216723/how-can-i-deserialize-an-enum-with-an-optional-internal-tag/61219284#61219284

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiObject {
    #[serde(rename = "type")]
    joi_type: MustBe!("object"),
    #[serde(default)]
    pub keys: BTreeMap<String, JoiDescribe>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiArray {
    #[serde(rename = "type")]
    joi_type: MustBe!("array"),
    #[serde(default)]
    pub items: Vec<JoiDescribe>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiAlternatives {
    #[serde(rename = "type")]
    joi_type: MustBe!("alternatives"),
    #[serde(default)]
    pub matches: Vec<AltSchema>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltSchema {
    pub schema: JoiDescribe,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiNumber {
    #[serde(rename = "type")]
    joi_type: MustBe!("number"),
    #[serde(default)]
    pub allow: Vec<serde_json::value::Number>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiString {
    #[serde(rename = "type")]
    joi_type: MustBe!("string"),
    #[serde(default)]
    pub allow: Vec<serde_json::value::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiDate {
    #[serde(rename = "type")]
    joi_type: MustBe!("date"),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiBoolean {
    #[serde(rename = "type")]
    joi_type: MustBe!("boolean"),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiAny {
    #[serde(rename = "type")]
    joi_type: MustBe!("any"),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiUnknown {
    #[serde(rename = "type")]
    pub joi_type: String,
}

/// The type specific joi describe options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum JoiDescribeType {
    Object(JoiObject),
    Array(JoiArray),
    Alternatives(JoiAlternatives),
    Date(JoiDate),
    Number(JoiNumber),
    String(JoiString),
    Boolean(JoiBoolean),
    Any(JoiAny),
    Unknown(JoiUnknown), // // Custom Variants
                         // NullableString {
                         //     #[serde(default)]
                         //     allow: Vec<serde_json::value::Value>,
                         // },
}
