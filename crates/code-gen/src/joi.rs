use genco::prelude::js;
use genco::prelude::*;
use serde::{self, Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// https://github.com/hapijs/joi/blob/7ead57a9f8180895e110f010b425ae411451bd08/lib/index.d.ts#L1316
// https://github.com/mrjono1/joi-to-typescript/blob/613e42022fb9847ab4c718410dbd980a457503ad/src/joiDescribeTypes.ts#LL10C56-L10C56

pub trait Tokenizer {
    fn to_tokens(&self) -> js::Tokens;
}

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
    Number {
        #[serde(default)]
        allow: Vec<serde_json::value::Number>,
    },
    String {
        #[serde(default)]
        allow: Vec<String>,
    },
    Boolean,
}

/// Representation of the `.describe()` response on a joi object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoiDescribe {
    #[serde(flatten)]
    type_options: JoiDescribeType,
    #[serde(default)]
    flags: JoiFlag,
    metas: Option<Vec<HashMap<String, String>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoiFlag {
    presence: Option<String>,
    description: Option<String>,
    #[serde(default)]
    only: bool, // default to false
}

impl Default for JoiFlag {
    fn default() -> Self {
        Self {
            presence: None,
            description: None,
            only: false,
        }
    }
}

impl Tokenizer for JoiFlag {
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

impl Tokenizer for JoiDescribe {
    fn to_tokens(&self) -> js::Tokens {
        let value: js::Tokens = match self.type_options {
            JoiDescribeType::Object {
                keys: ref collection,
            } => {
                let result = collection
                    .into_iter()
                    .map(|(key, value)| (key, value.to_tokens()));
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
                quote! { z.array($element) }
            }
            JoiDescribeType::Alternatives { ref matches } => {
                println!("Type {:?} : {:?}", &self, matches);
                unimplemented!()
            }
            JoiDescribeType::String { ref allow } => {
                if !self.flags.only {
                    quote! { z.string() }
                } else {
                    if allow.len() > 1 {
                        quote! { z.enum([$(for elem in allow join (, )=> $[str]($[const](elem)))]) }
                    } else if let Some(literal) = allow.get(0) {
                        quote! { z.literal($[str]($[const](literal))) }
                    } else {
                        unreachable!()
                    }
                }
            }
            JoiDescribeType::Date => {
                quote! { z.date() }
            }
            JoiDescribeType::Number { ref allow } => {
                if !self.flags.only {
                    quote! { z.number() }
                } else {
                    if allow.is_empty() {
                        unreachable!();
                    }
                    let elems = allow.iter().map(|elem| format!("{}", elem));
                    quote! { z.union([$(for elem in elems join (, )=> z.literal($elem))]) }
                }
            }
            JoiDescribeType::Boolean => {
                quote! { z.boolean() }
            }
        };

        let flag_tokens = self.flags.to_tokens().to_string().unwrap_or_default();

        // only append '.' if flag_tokens exists
        if !flag_tokens.is_empty() {
            quote! {
                $value.$flag_tokens
            }
        } else {
            value
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
        let tokens = joi.to_tokens();

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

        let tokens = joi.to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.array(z.string()).describe(\"A list of Test object\")".to_string())
        )
    }

    #[test]
    fn test_string_with_multiple_valid() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"{
            "type": "string",
            "flags": {
                "only": true
            },
            "allow": [
                "foo",
                "bar"
            ]
        }"#,
        )
        .unwrap();

        let tokens = joi.to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.enum([\"foo\", \"bar\"])".to_string())
        )
    }

    #[test]
    fn test_string_with_single_valid() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"{
            "type": "string",
            "flags": {
                "only": true
            },
            "allow": [
                "foo"
            ]
        }"#,
        )
        .unwrap();

        let tokens = joi.to_tokens();
        assert_eq!(tokens.to_string(), Ok("z.literal(\"foo\")".to_string()))
    }

    #[test]
    fn test_number_with_multiple_valid() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"{
            "type": "number",
            "flags": {
                "only": true
            },
            "allow": [
                3,
                4
            ]
        }"#,
        )
        .unwrap();

        let tokens = joi.to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.union([z.literal(3), z.literal(4)])".to_string())
        )
    }
}
