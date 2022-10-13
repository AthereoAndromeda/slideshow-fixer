mod zip;
pub use crate::zip::*;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::{self, DirEntry},
    path::Path,
};

#[cfg(not(target_arch = "wasm32"))]
pub fn write_files(entries: &Vec<DirEntry>, output_path: &Path) -> Result<(), std::io::Error> {
    for entry in entries {
        let new_path = output_path.join(entry.file_name());

        // This copies the entire file to memory. Use caution with big files
        let content = fs::read(entry.path())?;
        fs::write(new_path, content)?;
    }

    Ok(())
}
