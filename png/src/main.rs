use std::fs;
use std::mem;
use std::str;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

fn main() {
    let file = String::from("test.png");
    let mut cursor: u32 = 0;
    let sig = read_bytes_or_panic(&file, &mut cursor, 8);
    println!("Signature: ");
    print_bytes(&sig);
    assert!(
        sig == PNG_SIGNATURE,
        "ERROR: png file: {} seems not a valid png file.",
        file
    );

    loop {
        let length = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
        let mut binary = String::from("");
        for i in length {
            let bin = convert_decimal_to_binary(i);
            binary.push_str(&bin);
        }

        let size = convert_binary_to_decimal(String::from(binary));
        println!("Chunk size: {}", size);

        let chunk_type = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
        println!("Chunk type: {:#?}", str::from_utf8(&chunk_type).unwrap());

        // NOTE: skip chunk_data
        cursor = cursor + size;
        let chunk_crc = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
        // TODO: how to convert binary to hex?
        println!("Chunk crc: {:#08X?}", chunk_crc);
        println!("---------------");
    }
}

fn print_bytes(array: &[u8]) {
    for (_, i) in array.iter().enumerate() {
        print!("{} ", i);
    }
    println!();
}

fn convert_decimal_to_binary(mut input: u8) -> String {
    if input == 0 {
        return String::from("0000");
    }
    let mut tem_binary = String::from("");

    while input > 0 {
        if input % 2 == 1 {
            tem_binary.push('1');
        } else {
            tem_binary.push('0');
        }
        input = input / 2;
    }

    // fill up to 4 binary
    while tem_binary.len() as u32 % 4 != 0 {
        tem_binary.push('0');
    }

    // reverse
    let result: String = tem_binary.chars().rev().collect();

    return result;
}

fn convert_binary_to_decimal(input: String) -> u32 {
    let base: u32 = 2;
    let mut i: u32 = 0;
    let mut ans: u32 = 0;
    for n in input.chars().rev() {
        let number = n.to_digit(10);
        if let Some(num) = number {
            ans = ans + num * base.pow(i);
            i = i + 1;
        } else {
            panic!("Wrong format of the binary.")
        }
    }
    return ans;
}

fn read_bytes_or_panic(file_name: &String, start: &mut u32, internal: u32) -> Vec<u8> {
    let file = fs::read(file_name);
    let s = *start as usize;
    let e = (*start + internal) as usize;
    // moving cursor
    *start = *start + internal;

    if let Ok(content) = file {
        if *start > content.len() as u32 {
            panic!("Read to the end of the png file.")
        }
        return content[s..e].to_vec();
    } else {
        panic!("Can not open the png file: {}", file_name);
    }
}
