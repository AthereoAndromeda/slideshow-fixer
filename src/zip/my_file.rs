#[derive(Debug)]
pub struct MyFile {
    pub name: String,
    pub buf: Vec<u8>,
}

impl std::fmt::Display for MyFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} bytes)", self.name, self.buf.len())
    }
}
