use lurk::{
  eval::{empty_sym_env, Evaluator},
  proof::nova,
  store::{ContTag, Pointer, Store},
  writer::Write,
};
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Repl {
  store: Store<nova::S1>,
  limit: usize,
}

#[wasm_bindgen]
impl Repl {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Repl {
    Repl {
      store: Store::<nova::S1>::default(),
      limit: 100_000_000,
    }
  }

  /// Run a lurk snippet
  #[wasm_bindgen]
  pub fn execute_lurk(&mut self, source: JsValue) -> Result<JsValue, JsValue> {
    let expression = source
      .as_string()
      .ok_or_else(|| "input source must be a string")?;

    let mut context: HashMap<&str, String> = HashMap::new();

    context.insert("expression", expression.clone());
    if let Some(expr) = self.store.read(&expression) {
      match Evaluator::new(
        expr,
        empty_sym_env(&self.store),
        &mut self.store,
        self.limit,
      )
      .eval()
      {
        Ok((output, iterations, _)) => {
          let iterations_str = iterations.to_string();
          context.insert("iterations", iterations_str);
          let result_str = match output.cont.tag() {
            ContTag::Outermost | ContTag::Terminal => {
              let result = self
                .store
                .fetch(&output.expr)
                .clone()
                .ok_or_else(|| "fetch failed")?;
              result.fmt_to_string(&self.store)
            }
            ContTag::Error => "ERROR!".to_string(),
            _ => format!("Computation incomplete after limit: {}", self.limit),
          };

          context.insert("result", result_str);
        }
        Err(e) => {
          let error = format!("Evaluation Error: {}", &e);
          context.insert("result", error);
        }
      };
    } else {
      let error = format!("Syntax Error: {}", &expression);
      context.insert("result", error);
    }
    let json = json!(&context);
    Ok(json.to_string().into())
  }
}

#[cfg(test)]
mod tests {
  use anyhow::{anyhow, Result};
  use lurk::{
    eval::{empty_sym_env, Evaluator},
    proof::nova,
    store::{ContTag, Pointer, Store},
    writer::Write,
  };
  use serde_json::json;
  use std::collections::HashMap;

  pub struct Repl {
    store: Store<nova::S1>,
    limit: usize,
  }

  impl Repl {
    pub fn new() -> Repl {
      Repl {
        store: Store::<nova::S1>::default(),
        limit: 100_000_000,
      }
    }

    // Local test to mirror Lurk evaluation in browser
    pub fn run_lurk_local(&mut self, expression: String) -> Result<String, String> {
      let mut context: HashMap<&str, String> = HashMap::new();

      context.insert("expression", expression.clone());
      if let Some(expr) = self.store.read(&expression) {
        match Evaluator::new(
          expr,
          empty_sym_env(&self.store),
          &mut self.store,
          self.limit,
        )
        .eval()
        {
          Ok((output, iterations, _)) => {
            let iterations_str = iterations.to_string();
            context.insert("iterations", iterations_str);
            let result_str = match output.cont.tag() {
              ContTag::Outermost | ContTag::Terminal => match self.store.fetch(&output.expr) {
                Some(val) => val.clone().fmt_to_string(&self.store),
                None => "fetch failed".to_string(),
              },
              ContTag::Error => "ERROR!".to_string(),
              _ => format!("Computation incomplete after limit: {}", self.limit),
            };

            context.insert("result", result_str);
          }
          Err(e) => {
            let error = format!("Evaluation Error: {}", &e);
            context.insert("result", error);
          }
        };
      } else {
        let error = format!("Syntax Error: {}", &expression);
        context.insert("result", error);
      }
      let json = json!(&context);
      Ok(json.to_string().into())
    }

    // Native Lurk test
    pub fn run_lurk_native(&mut self, expression: &str) -> Result<String> {
      let expr = self
        .store
        .read(expression)
        .ok_or_else(|| anyhow!("read failed"))?;
      let mut evaluator = Evaluator::new(
        expr,
        empty_sym_env(&self.store),
        &mut self.store,
        self.limit,
      );
      let (output, _, _) = evaluator.eval().map_err(|e| anyhow!("{}", e))?;
      match output.cont.tag() {
        ContTag::Outermost | ContTag::Terminal => Ok(
          self
            .store
            .fetch(&output.expr)
            .ok_or_else(|| anyhow!("fetch failed"))?
            .clone()
            .fmt_to_string(&self.store),
        ),
        ContTag::Error => Err(anyhow!("Continuation Error")),
        _ => Err(anyhow!(
          "Computation incomplete after limit: {}",
          self.limit
        )),
      }
    }
  }

  #[test]
  fn open_commit_roundtrip() {
    let mut repl = Repl::new();
    let input = "123";
    let expr = format!("(commit {})", input);
    let ptr = repl.run_lurk_native(&expr).unwrap();
    let open = format!("(open {})", ptr);
    assert_eq!(input, repl.run_lurk_native(&open).unwrap());
  }

  #[test]
  #[should_panic = "hidden value could not be opened"]
  fn open_opaque_commit_native() {
    let mut repl = Repl::new();
    let expr = "(open 123)";
    repl.run_lurk_native(expr).unwrap();
  }

  // Should succeed with an error string message
  #[test]
  fn open_opaque_commit_local() {
    let mut repl = Repl::new();
    let lurk_expr: String = "(open 123)".into();
    let output = repl.run_lurk_local(lurk_expr).unwrap();
    println!("{}", output);
  }
}
