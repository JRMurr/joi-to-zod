use std::collections::HashMap;

use serde::{self, Deserialize, Serialize};

// https://github.com/hapijs/joi/blob/7ead57a9f8180895e110f010b425ae411451bd08/lib/index.d.ts#L1316
// https://github.com/mrjono1/joi-to-typescript/blob/613e42022fb9847ab4c718410dbd980a457503ad/src/joiDescribeTypes.ts#LL10C56-L10C56
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum JoiDescribe {
    Object {
        keys: Option<HashMap<String, JoiDescribe>>,
    },
    Date,
    Number,
    String,
    Boolean,
}

#[cfg(test)]
mod tests {
    use super::JoiDescribe;

    #[test]
    fn test_basic_parse() {
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
}
