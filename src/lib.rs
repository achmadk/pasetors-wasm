/// modules related to PASETO v4
// pub mod v4;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(string: String);
}
/// exported `greet` method 
#[wasm_bindgen]
pub fn greet(name: &str) {
  log(format!("Hello, {:?}", name))
}
