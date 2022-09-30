pub mod repl;
mod utils;

/// Module for wasm-bindgen specific handling and endpoints.
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;

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
  #[cfg(feature = "debug")]
  console_error_panic_hook::set_once();
  Ok(())
}
