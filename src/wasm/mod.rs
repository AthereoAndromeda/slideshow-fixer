use crate::zip_main;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn aaa() {
    alert("amongus");
}

#[wasm_bindgen]
pub fn init_module() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn js_write_files(file: &[u8]) -> Box<[u8]> {
    // alert(format!("{:?}", file).as_str());
    let a = file.len().to_string();
    alert(a.as_str());

    let zip_file = Cursor::new(file);
    let extracted = zip_main(zip_file).unwrap();
    let inner = extracted.into_inner();

    alert("woaah");

    inner.into_boxed_slice()
}
