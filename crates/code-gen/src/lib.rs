mod joi;

use crate::joi::JoiDescribe;

pub fn gen_from_file(contents: String) {
    let joi_str: JoiDescribe= serde_json::from_str(contents.as_str()).expect("Something");
    joi_str.to_tokens();

}

pub fn gen() -> String {
    unimplemented!()
}

