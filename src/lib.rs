pub mod zip;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
use std::{fs::DirEntry, path::Path};

#[cfg(not(target_arch = "wasm32"))]
pub fn write_files(entries: &Vec<DirEntry>, output_path: &Path) -> Result<(), std::io::Error> {
    use std::{
        fs::File,
        io::{BufReader, BufWriter, Read, Write},
    };

    // 100kb
    let mut buf = vec![0u8; 1024 * 100];

    for entry in entries {
        let new_path = output_path.join(entry.file_name());

        #[cfg(debug_assertions)]
        dbg!(&new_path);

        // 20 MB buffer
        const BUFREAD_BUFFER_SIZE: usize = 1024 * 1000 * 20;

        // 100 KB buffer
        const BUFWRITER_BUFFER_SIZE: usize = 1024 * 100;

        let file = File::open(entry.path())?;
        let mut reader = BufReader::with_capacity(BUFREAD_BUFFER_SIZE, file);

        let output_file = File::create(new_path)?;
        let mut writer = BufWriter::with_capacity(BUFWRITER_BUFFER_SIZE, output_file);

        while let Ok(bytes) = reader.read(&mut buf) {
            writer.write(&mut buf)?;

            if bytes == 0 {
                break;
            }
        }
    }

    Ok(())
}
