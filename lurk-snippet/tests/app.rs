//use wasm_bindgen::JsValue;
//use web_sys::console;
//use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasm_bindgen_test::wasm_bindgen_test_configure;

extern crate lurk_snippet;
//use lurk_snippet::repl::Repl;

wasm_bindgen_test_configure!(run_in_browser);

// This runs a unit test in native Rust, so it can only use Rust APIs.
// I.e. won't work with any wasm_bindgen functions
// See src/repl.rs for equivalent unit tests
#[test]
fn rust_test() {
    assert_eq!(1, 1);
}

// This runs a unit test in the browser, so it can use browser APIs.
// Currently broken due to lack of Rust-initialized thread pool
//#[wasm_bindgen_test]
//fn web_test() {
//  let mut repl = Repl::new();
//  let lurk_expr: JsValue = "(open 123)".into();
//  let output = repl.execute_lurk(lurk_expr).unwrap();
//  console::log_1(&output);
//}
