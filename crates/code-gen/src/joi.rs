use genco::prelude::js;
use genco::prelude::*;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

// https://github.com/hapijs/joi/blob/7ead57a9f8180895e110f010b425ae411451bd08/lib/index.d.ts#L1316
// https://github.com/mrjono1/joi-to-typescript/blob/613e42022fb9847ab4c718410dbd980a457503ad/src/joiDescribeTypes.ts#LL10C56-L10C56

/// The type specfic joi describe options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum JoiDescribeType {
    Object {
        keys: Option<HashMap<String, JoiDescribe>>,
    },
    Array {
        items: Option<Vec<JoiDescribe>>,
    },
    Alternatives {
        matches: Option<Vec<HashMap<String, String>>>,
    },
    Date,
    Number,
    String,
    Boolean,
}

/// Representation of the `.describe()` response on a joi object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiDescribe {
    #[serde(flatten)]
    type_options: JoiDescribeType,
    flags: Option<JoiFlag>,
    metas: Option<Vec<HashMap<String, String>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoiFlag {
    presense: Option<String>,
    description: Option<String>,
}

impl Tokenizer<js::Tokens> for JoiFlag {
    fn to_tokens(&self) -> js::Tokens {
        let description: js::Tokens = self
            .description
            .as_ref()
            .map(|desc| {
                quote! {
                    describe($[str]($[const](desc)))
                }
            })
            .unwrap_or_default();

        // TODO: prescenc
        description
    }
}

pub trait Tokenizer<T> {
    fn to_tokens(&self) -> T;
}

impl Tokenizer<js::Tokens> for JoiDescribe {
    fn to_tokens(&self) -> js::Tokens {
        let value: js::Tokens = match self.type_options {
            JoiDescribeType::Object {
                keys: ref collection,
            } => {
                println!("Type {:?} : {:?}", &self, collection);
                if let Some(children) = collection {
                    for (key, value) in children.into_iter() {
                        println!("Key: {}, value: {:?}", key, value);
                    }
                }
                unimplemented!()
            }
            JoiDescribeType::Array { ref items } => {
                println!("Type {:?} : {:?}", &self, items);
                unimplemented!()
            }
            JoiDescribeType::Alternatives { ref matches } => {
                println!("Type {:?} : {:?}", &self, matches);
                unimplemented!()
            }
            JoiDescribeType::String => {
                quote! {
                    z.string()
                }
            }
            JoiDescribeType::Date => {
                quote! {
                    z.date()
                }
            }
            JoiDescribeType::Number => {
                quote! {
                    z.number()
                }
            }
            JoiDescribeType::Boolean => {
                quote! {
                    z.boolean()
                }
            }
        };
        match self.flags {
            Some(ref flags) => {
                let flag_tokens = flags.to_tokens();
                quote! {
                    $value.$flag_tokens
                }
            }
            None => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{JoiDescribe, Tokenizer};

    #[test]
    fn test_convert_single_number() {
        let describe = r#"{
            "type": "number",
            "flags": {
                "description": "some description"
            },
            "rules": [
                {
                    "name": "integer"
                }
            ]
        }"#;

        let joi: JoiDescribe = serde_json::from_str(describe).expect("should work...");
        let tokens = dbg!(joi).to_tokens();

        assert_eq!(
            tokens.to_string(),
            Ok("z.number().describe(\"some description\")".to_string())
        )
    }

    #[test]
    fn test_basic_parse_object() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"
        {
            "type": "object",
            "keys": {
                "name": {
                    "type": "string",
                    "flags": {
                        "presence": "optional",
                        "description": "Test Schema Name"
                    }
                },
                "propertyName1": {
                    "type": "boolean",
                    "flags": {
                        "presence": "required"
                    }
                },
                "dateCreated": {
                    "type": "date"
                },
                "count": {
                    "type": "number"
                },
                "int": {
                    "type": "number",
                    "flags": {
                        "presence": "optional"
                    },
                    "rules": [
                        {
                            "name": "integer"
                        }
                    ]
                },
                "obj": {
                    "type": "object"
                }
            }
        }
        "#,
        )
        .unwrap();

        dbg!(joi);
    }

    #[test]
    fn test_basic_parse_array() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"
            {
                "type": "array",
                "flags": {
                  "presence": "required",
                  "description": "A list of Test object"
                },
                "metas": [
                  {
                    "className": "TestList"
                  }
                ],
                "items": [
                  {
                    "type": "string"
                  }
                ]
              }
        "#,
        )
        .unwrap();

        dbg!(joi);
    }
}
