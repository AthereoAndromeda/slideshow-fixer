#![allow(dead_code)]
use super::zip::{run_process, MyZipError};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

impl Into<JsValue> for MyZipError {
    fn into(self) -> JsValue {
        let s = self.to_string();
        JsValue::from_str(&s)
    }
}

// Default allocator for WASM
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// // Import the `window.alert` function from the Web.
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn attach_panic_hook() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn js_write_files(file: &[u8]) -> Result<Box<[u8]>, MyZipError> {
    let zip_file = Cursor::new(file);
    let extracted = run_process(zip_file)?;
    Ok(extracted)
}
