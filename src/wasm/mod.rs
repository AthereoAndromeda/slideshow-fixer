#![allow(dead_code)]
use crate::zip_main;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Default allocator for WASM
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init_module() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn js_write_files(file: &[u8]) -> Box<[u8]> {
    let zip_file = Cursor::new(file);
    let extracted = zip_main(zip_file).unwrap();
    let inner = extracted.into_inner();

    inner.into_boxed_slice()
}
