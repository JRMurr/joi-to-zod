use napi_derive::napi;

use code_gen::gen;

#[napi]
pub fn run() -> String {
  gen()
}
