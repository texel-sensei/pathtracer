use std::fs;
use std::io::prelude::*;

pub fn write_ppm(filename: &str, size: (u32, u32), data: &[u8]) -> std::io::Result<()> {
    let mut file = fs::File::create(filename)?;

    // write header
    write!(file, "P6\n{} {}\n255\n", size.0, size.1)?;

    file.write(data)?;

    Ok(())
}
