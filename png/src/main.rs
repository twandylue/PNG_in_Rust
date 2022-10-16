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

    let lens = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
    println!("Length: ");
    print_bytes(&lens);
    let mut binary = String::from("");
    for i in lens {
        let bin = convert_decimal_to_binary(i);
        let rev_bin = reverse_binary(&bin);
        binary.push_str(&rev_bin);
    }

    let number = convert_binary_to_decimal(String::from(binary));
    println!("Length: {}", number);

    let chunk_type = read_bytes_or_panic(&file, &mut cursor, mem::size_of::<u32>() as u32);
    println!("Chunk type: ");
    print_bytes(&chunk_type);
    println!("Chunk type: {:#?}", str::from_utf8(&chunk_type).unwrap());
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
    let mut output = String::from("");

    while input > 0 {
        if input % 2 == 1 {
            output.push('1');
        } else {
            output.push('0');
        }
        input = input / 2;
    }
    return output;
}

fn convert_binary_to_decimal(input: String) -> u32 {
    let mut i: u32 = 0;
    let base: u32 = 2;
    let mut ans: u32 = 0;
    let t: String = input.chars().rev().collect();
    for n in t.chars() {
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

fn reverse_binary(input: &String) -> String {
    let result: String = input.chars().rev().collect();
    return result;
}

fn read_bytes_or_panic(file_name: &String, start: &mut u32, internal: u32) -> Vec<u8> {
    let file = fs::read(file_name);
    let s = *start as usize;
    let e = (*start + internal) as usize;
    // moving cursor
    *start = *start + internal;

    if let Ok(content) = file {
        return content[s..e].to_vec();
    } else {
        panic!("Can not open the png file: {}", file_name);
    }
}
