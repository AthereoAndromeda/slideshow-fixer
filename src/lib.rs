use std::{
    fs::{self, DirEntry},
    path::Path,
};

mod zip;
pub use crate::zip::*;

use wasm_bindgen::prelude::wasm_bindgen;

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

#[wasm_bindgen]
pub fn js_write_files(file: &[u8]) -> u8 {
    file[0]
}
