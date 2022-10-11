use std::{
    fs::{self, DirEntry},
    io::Cursor,
    path::Path,
};

mod zip;
pub use crate::zip::*;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
pub fn write_files(entries: &Vec<DirEntry>, output_path: &Path) -> Result<(), std::io::Error> {
    for entry in entries {
        let new_path = output_path.join(entry.file_name());

        // This copies the entire file to memory. Use caution with big files
        let content = fs::read(entry.path())?;
        fs::write(new_path, content)?;
    }

    Ok(())
}

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
pub fn js_write_files(file: &[u8]) -> Box<[u8]> {
    let zip_file = Cursor::new(file);
    let extracted = zip_main(zip_file).unwrap();
    let inner = extracted.into_inner();

    alert("woaah");

    inner.into_boxed_slice()
}
