use blstrs::Scalar as Fr;
use lurk::eval::{empty_sym_env, Evaluator};
use lurk::store::{ContTag, Pointer, Store};
use lurk::writer::Write;
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

pub struct LurkRepl {
  store: Store<Fr>,
  limit: usize,
}

impl LurkRepl {
  pub fn new() -> Self {
    Self {
      store: Store::<Fr>::default(),
      limit: 100_000_000,
    }
  }

  pub fn execute_lurk(&mut self, expression: &str) -> Result<String, JsValue> {
    let mut context: HashMap<&str, String> = HashMap::new();

    context.insert("expression", expression.to_string());
    if let Some(expr) = self.store.read(expression) {
      let (output, iterations, _) = Evaluator::new(
        expr,
        empty_sym_env(&self.store),
        &mut self.store,
        self.limit,
      )
      .eval();

      let iterations_str = iterations.to_string();
      context.insert("iterations", iterations_str);
      let result_str = match output.cont.tag() {
        ContTag::Outermost | ContTag::Terminal => {
          let result = self.store.fetch(&output.expr).unwrap();
          result.fmt_to_string(&self.store)
        }
        ContTag::Error => "ERROR!".to_string(),
        _ => format!("Computation incomplete after limit: {}", self.limit),
      };

      context.insert("result", result_str);
    } else {
      let error = format!("Syntax Error: {}", &expression);
      context.insert("result", error);
    }
    let json = json!(&context);
    Ok(json.to_string())
  }
}
