use std::{
    fs::{self, DirEntry},
    path::Path,
};

mod zip;
pub use crate::zip::*;

#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(target_family = "wasm")]
extern crate wee_alloc;

// Default allocator for WASM
#[cfg(target_family = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn write_files(entries: &Vec<DirEntry>, output_path: &Path) -> Result<(), std::io::Error> {
    for entry in entries {
        let new_path = output_path.join(entry.file_name());

        // This copies the entire file to memory. Use caution with big files
        let content = fs::read(entry.path())?;
        fs::write(new_path, content)?;
    }

    Ok(())
}
