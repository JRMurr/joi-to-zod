use genco::prelude::js;
use genco::prelude::*;
use serde::{self, Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// https://github.com/hapijs/joi/blob/7ead57a9f8180895e110f010b425ae411451bd08/lib/index.d.ts#L1316
// https://github.com/mrjono1/joi-to-typescript/blob/613e42022fb9847ab4c718410dbd980a457503ad/src/joiDescribeTypes.ts#LL10C56-L10C56

/// The type specific joi describe options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum JoiDescribeType {
    Object {
        #[serde(default)]
        keys: BTreeMap<String, JoiDescribe>,
    },
    Array {
        #[serde(default)]
        items: Vec<JoiDescribe>,
    },
    Alternatives {
        #[serde(default)]
        matches: Vec<HashMap<String, String>>,
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
                let mut result = BTreeMap::new();
                for (key, value) in collection.into_iter() {
                    result.insert(key, value.to_tokens());
                }
                quote! {
                    z.object({$(for (key, value) in result join (, )=> $key: $value)})
                }
            }
            JoiDescribeType::Array { ref items } => {
                let mut children = items.iter().map(|child| child.to_tokens());
                let element = if children.len() > 1 {
                    // not sure how common multiple array items is but i guess we wrap in union?
                    quote! {
                        z.union([$(for child in children join (, )=> $child)])
                    }
                } else {
                    children.next().unwrap()
                };
                quote! {
                    z.array($element)
                }
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

                let flag_tokens = match flag_tokens.to_string() {
                    Ok(res) => {
                        // only append '.' if flag_tokens exists
                        if !res.is_empty() {
                            quote! {
                                .$flag_tokens
                            }
                        } else {
                            quote!{}
                        }
                    },
                    Err(_) => quote!{},
                };
                quote! {
                    $value$flag_tokens
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

        let tokens = joi.to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.object({count: z.number(), dateCreated: z.date(), int: z.number(), name: z.string().describe(\"Test Schema Name\"), obj: z.object({}), propertyName1: z.boolean()})".to_string())
        )
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

        let tokens = dbg!(joi).to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.array(z.string()).describe(\"A list of Test object\")".to_string())
        )
    }
}
