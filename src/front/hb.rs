use rocket_contrib::templates::handlebars::{
  Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use serde_json::value::Value;

pub fn plural(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> HelperResult {
  let singular = h.param(0);
  let plural = h.param(1);
  let quantity = h.param(2);

  if let (Some(singular), Some(plural), Some(quantity)) = (singular, plural, quantity) {
    if let Value::Number(quantity) = quantity.value() {
      if let Some(quantity) = quantity.as_u64() {
        if quantity > 1 {
          out.write(&plural.value().render())?;
        } else {
          out.write(&singular.value().render())?;
        }
      }
    }
  }
  Ok(())
}