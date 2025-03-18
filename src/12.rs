use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("some_file.txt");
    let mut file = fs::File::create(path)?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
