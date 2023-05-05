use genco::prelude::js;
use genco::prelude::*;
use serde::{self, Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// https://github.com/hapijs/joi/blob/7ead57a9f8180895e110f010b425ae411451bd08/lib/index.d.ts#L1316
// https://github.com/mrjono1/joi-to-typescript/blob/613e42022fb9847ab4c718410dbd980a457503ad/src/joiDescribeTypes.ts#LL10C56-L10C56

pub trait Tokenizer {
    fn to_tokens(&self) -> js::Tokens;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltSchema {
    schema: JoiDescribe,
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
        matches: Vec<AltSchema>,
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

        let presence: js::Tokens = self
            .presence
            .as_ref()
            .map(|pres| {
                let str = match pres.as_str() {
                    "optional" => quote! {
                        optional()
                    },
                    "required" => quote! {
                        required()
                    },
                    "forbidden" => quote! {
                        undefined()
                    },
                    _ => unreachable!(),
                };
                str
            })
            .unwrap_or_default();

        // description
        let mut flag_tokens = Vec::new();
        if !description.is_empty() {
            flag_tokens.push(description);
        }

        // presence
        // in joi - everything is optional, in zod - everything is required
        // so gotta add .optional() to everything that does not have a presence
        // and ignore .required() presences
        if &presence.to_string().unwrap_or_default() == "required()" {
            // no op
        } else if !presence.is_empty() {
            flag_tokens.push(presence);
        } else {
            flag_tokens.push(quote! {
                optional()
            });
        }

        quote! {
            $(for flag in flag_tokens.iter() join (. )=> $flag)
        }
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
                quote! {
                    z.union([$(for one_match in matches.iter() join (, )=> $(one_match.schema.to_tokens()))])
                }
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

        let flag_tokens = self.flags.to_tokens();

        // only append '.' if flag_tokens exists
        if flag_tokens.is_empty() {
            value
        } else {
            quote! {
                $value.$flag_tokens
            }
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
            Ok("z.number().describe(\"some description\").optional()".to_string())
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
                    "type": "date",
                    "flags": {
                        "presence": "required"
                    }
                },
                "count": {
                    "type": "number",
                    "flags": {
                        "presence": "required"
                    }
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
                },
                "yuck": {
                    "type": "string",
                    "flags": {
                        "presence": "forbidden"
                    }
                }
            }
        }
        "#,
        )
        .unwrap();

        let tokens = joi.to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.object({count: z.number(), dateCreated: z.date(), int: z.number().optional(), name: z.string().describe(\"Test Schema Name\").optional(), obj: z.object({}).optional(), propertyName1: z.boolean(), yuck: z.string().undefined()}).optional()".to_string())
        )
    }

    #[test]
    fn test_basic_parse_array() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"
            {
                "type": "array",
                "flags": {
                  "presence": "optional",
                  "description": "A list of Test object"
                },
                "metas": [
                  {
                    "className": "TestList"
                  }
                ],
                "items": [
                  {
                    "type": "string",
                    "flags": {
                        "presence": "required"
                    }
                  }
                ]
              }
        "#,
        )
        .unwrap();

        let tokens = joi.to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.array(z.string()).describe(\"A list of Test object\").optional()".to_string())
        )
    }

    #[test]
    fn test_string_with_multiple_valid() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"{
            "type": "string",
            "flags": {
                "presence": "required",
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
                "presence": "required",
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
                "presence": "required",
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

    #[test]
    fn test_convert_simple_alternative() {
        let joi: JoiDescribe = serde_json::from_str(
            r#"
        {
            "type":"alternatives",
            "matches":[
                {
                    "schema":{
                        "type":"number"
                    }
                },
                {
                    "schema":{
                        "type":"string"
                    }
                }
            ]
        }
        "#,
        )
        .unwrap();

        let tokens = dbg!(joi).to_tokens();
        assert_eq!(
            tokens.to_string(),
            Ok("z.union([z.number(), z.string()])".to_string())
        )
    }
}
