use std::{collections::{HashMap}};

use genco::prelude::js;
use serde::{self, Deserialize, Serialize};

// https://github.com/hapijs/joi/blob/7ead57a9f8180895e110f010b425ae411451bd08/lib/index.d.ts#L1316
// https://github.com/mrjono1/joi-to-typescript/blob/613e42022fb9847ab4c718410dbd980a457503ad/src/joiDescribeTypes.ts#LL10C56-L10C56
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum JoiDescribe {
    Object(JoiCollection),
    Array(JoiCollection),
    Alternatives(JoiCollection),
    Date(JoiItem),
    Number(JoiItem),
    String(JoiItem),
    Boolean(JoiItem),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoiItem {
    flags: Option<JoiFlag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoiCollection {
    flags: Option<JoiFlag>,
    metas: Option<Vec<HashMap<String, String>>>,
    keys: Option<HashMap<String, JoiDescribe>>,
    items: Option<Vec<JoiDescribe>>,
    matches: Option<Vec<HashMap<String, String>>>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoiFlag {
    presense: Option<String>, 
    description: Option<String>, 
}

pub trait Tokenizer<T> {
    fn to_tokens(&self) -> Option<Vec<T>>;
}

impl Tokenizer<js::Tokens> for JoiDescribe {
    fn to_tokens(&self) -> Option<Vec<js::Tokens>> {
        match self {
            Self::Object(collection) => {
                println!("Type {:?} : {:?}", &self, collection);
                let keys = collection.keys.clone()?;
                for (key, value) in keys.into_iter() {
                   println!("Key: {}, value: {:?}", key, value);
                }
                unimplemented!()
            }
            Self::Array(collection) => {
                println!("Type {:?} : {:?}", &self, collection);
                let items = collection.items.clone()?
                unimplemented!()
            },
            Self::Alternatives(collection) => {
                println!("Type {:?} : {:?}", &self, collection);
                unimplemented!()
            },
            Self::String(item) => {
                println!("{:?}", item);
                unimplemented!()
            },
            Self::Date(item) => {
                println!("{:?}", item);
                unimplemented!()
            },
            Self::Number(item) => {
                println!("{:?}", item);
                unimplemented!()
            },
            Self::Boolean(item) => {
                println!("{:?}", item);
                unimplemented!()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JoiDescribe;

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
