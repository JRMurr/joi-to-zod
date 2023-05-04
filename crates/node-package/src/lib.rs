use napi::{bindgen_prelude::*, JSON};
use napi_derive::napi;

use code_gen::gen;

#[napi]
pub fn to_zod(env: Env, joi_schema: Object) -> napi::Result<String> {
  let describe_obj = match joi_schema.get::<&str, JsFunction>("describe") {
    Ok(Some(func)) => func
      .call_without_args(Some(&joi_schema))?
      .coerce_to_object()?,
    _ => joi_schema,
  };
  let json: JSON = env.get_global()?.get_named_property_unchecked("JSON")?;
  Ok(
    gen(json.stringify(describe_obj)?)
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("{}", e)))?,
  )
}
