use wasm_bindgen::prelude::*;

use blstrs::Scalar as Fr;
use lurk::{
    eval::{empty_sym_env, Evaluator},
    store::{ContTag, Pointer, Store},
    writer::Write,
};
use serde_json::json;
use std::collections::HashMap;

pub use wasm_bindgen_rayon::init_thread_pool;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub struct Repl {
    store: Store<Fr>,
    limit: usize,
}

#[wasm_bindgen]
impl Repl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Repl {
        Repl {
            store: Store::<Fr>::default(),
            limit: 100_000_000,
        }
    }

    /// Run a lurk command
    #[wasm_bindgen]
    pub fn execute_lurk(&mut self, source: JsValue) -> Result<JsValue, JsValue> {
        let expression = source
            .as_string()
            .ok_or_else(|| "input source must be a string")?;

        let mut context: HashMap<&str, String> = HashMap::new();
        context.insert("expression", expression.clone());
        if let Some(expr) = self.store.read(&expression) {
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
                    let result = self.store.fetch(&output.expr).clone().unwrap();
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
        Ok(json.to_string().into())
    }
}
