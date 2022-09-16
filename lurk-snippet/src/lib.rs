mod utils;
/// Module for wasm-bindgen specific handling and endpoints.
use lurk::{
    eval::{empty_sym_env, Evaluator},
    store::{ContTag, Pointer, Store},
    writer::Write,
};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde_json::json;
use blstrs::Scalar as Fr;

use std::panic;
// use web_sys::console;

pub use wasm_bindgen_rayon::init_thread_pool;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_panic_hook() {
    utils::set_panic_hook();
}

#[wasm_bindgen(module = "/js/panic-handler.js")]
extern "C" {
    fn onPanic(message: String);
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    /* 
    panic::set_hook(Box::new(|panic_info| {
        let msg = format!("{}", panic_info);
        console::log_1(&JsValue::from_str(&msg));
        onPanic(msg);
    }));
    */
    Ok(())
}

#[wasm_bindgen]
pub struct Repl {
    store: Store<Fr>,
}

#[wasm_bindgen]
impl Repl {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Repl {
        Repl { store: Store::<Fr>::default() }
    }

    /// Run a lurk snippet
    #[wasm_bindgen]
    pub fn execute_lurk(&mut self, source: JsValue) -> Result<JsValue, JsValue> {
        let limit = 100_000_000;

        let expression = source
            .as_string()
            .ok_or_else(|| "input source must be a string")?;

        //let mut store = Store::<Fr>::default();
        let mut context: HashMap<&str, String> = HashMap::new();

        context.insert("expression", expression.clone());
        if let Some(expr) = self.store.read(&expression) {
            let (output, iterations, _) =
                Evaluator::new(expr, empty_sym_env(&self.store), &mut self.store, limit).eval();

            let iterations_str = iterations.to_string();
            context.insert("iterations", iterations_str);
            let result_str = match output.cont.tag() {
                ContTag::Outermost | ContTag::Terminal => {
                    let result = self.store.fetch(&output.expr).clone().unwrap();
                    result.fmt_to_string(&self.store)
                }
                ContTag::Error => "ERROR!".to_string(),
                _ => format!("Computation incomplete after limit: {}", limit),
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
