use png::models::png_file::BytePacketBuffer;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::str;

const FILE_NAME: &str = "test.png";
const OUTPUT_FILE_NAME: &str = "test_copy.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = fs::File::open(FILE_NAME)?;
    let mut buffer = BytePacketBuffer::new();
    f.read_to_end(&mut buffer.buf)?;

    let mut output_f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(OUTPUT_FILE_NAME)
        .unwrap();

    let sig = buffer.get_range(0, 8)?;
    output_f.write(sig)?;
    assert!(
        sig == buffer.signature(),
        "ERROR: png file: {} seems not a valid png file.",
        FILE_NAME
    );
    println!("File Name: {}", FILE_NAME);
    println!("Signature[bytes]:\n{:?}", sig);
    buffer.step(8)?;

    let mut pause = false;
    while !pause {
        let length = buffer.read_u32()?;
        output_f.write(&length.to_be_bytes())?;
        println!("Chunk size: {}", length);

        let chunk_type = buffer.read_u32()?;
        output_f.write(&chunk_type.to_be_bytes())?;
        if format!("{chunk_type:#X}") == "0x49454E44" {
            pause = true;
        }
        println!(
            "Chunk type: {:#?} ({})",
            str::from_utf8(&chunk_type.to_be_bytes()).unwrap(),
            format!("{chunk_type:#x}")
        );

        // WARN: skip chunk_data
        let chunk_data = buffer.get_range(buffer.pos(), length as usize)?;
        output_f.write(chunk_data)?;
        buffer.step(length as usize)?;

        let chunk_crc = buffer.read_u32()?;
        output_f.write(&chunk_crc.to_be_bytes())?;
        println!("Chunk crc: {:#X}", chunk_crc);
        println!("---------------");
    }
    println!("***End***");

    Ok(())
}
