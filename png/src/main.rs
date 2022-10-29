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

    let mut pause = false;
    while !pause {
        let length = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
        let mut binary = String::from("");
        for i in length {
            let bin = convert_decimal_to_binary(i);
            binary.push_str(&bin);
        }

        let size = convert_binary_to_decimal(String::from(binary));
        println!("Chunk size: {}", size);

        let chunk_type = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
        println!(
            "Chunk type: {:#?} (0x{})",
            str::from_utf8(&chunk_type).unwrap(),
            convert_to_hex(&chunk_type)
        );

        // WARN: skip chunk_data
        cursor = cursor + size;
        let chunk_crc = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
        println!("Chunk crc: 0x{}", convert_to_hex(&chunk_crc));

        if format!("0x{}", convert_to_hex(&chunk_type)) == "0x49454E44" {
            pause = true;
        }
        println!("---------------");
    }
    println!("***End***")
}

fn convert_to_hex(v: &Vec<u8>) -> String {
    let mut result = String::from("");
    for c in v {
        result.push_str(&format!("{:X?}", c))
    }

    return result;
}

fn print_bytes(array: &[u8]) {
    for (_, i) in array.iter().enumerate() {
        print!("{} ", i);
    }
    println!();
}

fn convert_decimal_to_binary(mut input: u8) -> String {
    if input == 0 {
        return String::from("00000000");
    }
    let mut result = String::from("");

    while input > 0 {
        if input % 2 == 1 {
            result.push('1');
        } else {
            result.push('0');
        }
        input = input / 2;
    }

    // fill up to 8-bit binary
    while result.len() as u32 % 8 != 0 {
        result.push('0');
    }

    // reverse
    return result.chars().rev().collect();
}

fn convert_binary_to_decimal(input: String) -> u32 {
    let base: u32 = 2;
    let mut i: u32 = input.len() as u32 - 1;
    let mut ans: u32 = 0;
    for n in input.chars() {
        let number = n.to_digit(10);
        if let Some(num) = number {
            ans = ans + num * base.pow(i);
            if i == 0 {
                break;
            }
            i = i - 1;
        } else {
            panic!("Wrong format of the binary.")
        }
    }
    return ans;
}

/*
* NOTE: some references:
    1. https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    2. https://phaiax.github.io/mdBook/rustbook/ch08-01-vectors.html
    3. https://www.reddit.com/r/learnrust/comments/9a5mfg/why_do_i_need_to_dereference_a_string_in_order_to/
*/
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
