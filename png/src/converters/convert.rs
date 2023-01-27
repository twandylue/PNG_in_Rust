pub fn convert_byte_to_binary_in_string(mut input: u8) -> String {
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

pub fn convert_binary_to_decimal(input: String) -> u32 {
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

/// convert array of bytes to binary in String
pub fn convert_bytes_to_binary_in_string(input: Vec<u8>) -> String {
    let mut output = String::new();
    for i in input {
        output.push_str(&format!("{i:08b}"));
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::{
        convert_binary_to_decimal, convert_byte_to_binary_in_string,
        convert_bytes_to_binary_in_string,
    };

    #[test]
    fn convert_byte_to_binary_in_string_1() {
        // arrange
        let input: u8 = 0;
        // let expected = format!("{:0<8}", "");
        let expected = String::from("00000000");

        // act
        let actual = convert_byte_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_byte_to_binary_in_string_2() {
        // arrange
        let input: u8 = 1;
        let expected = String::from("00000001");

        // act
        let actual = convert_byte_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_byte_to_binary_in_string_3() {
        // arrange
        let input: u8 = 255;
        let expected = String::from("11111111");

        // act
        let actual = convert_byte_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_byte_to_binary_in_string_4() {
        // arrange
        let input: u8 = 55;
        let expected = String::from("00110111");

        // act
        let actual = convert_byte_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_binary_to_decimal_ok_1() {
        // arrange
        let input = String::from("00110111");
        let expected: u32 = 55;

        // act
        let actual = convert_binary_to_decimal(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_binary_to_decimal_ok_2() {
        // arrange
        let input = String::from("11111111");
        let expected: u32 = 255;

        // act
        let actual = convert_binary_to_decimal(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_binary_to_decimal_ok_3() {
        // arrange
        let input = String::from("00000000");
        let expected: u32 = 0;

        // act
        let actual = convert_binary_to_decimal(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_binary_to_decimal_ok_4() {
        // arrange
        let input = String::from("0010010111");
        let expected: u32 = 151;

        // act
        let actual = convert_binary_to_decimal(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_binary_to_decimal_ok_5() {
        // arrange
        let input = String::from("001");
        let expected: u32 = 1;

        // act
        let actual = convert_binary_to_decimal(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_bytes_to_binary_in_string_ok_1() {
        // arrange
        let input: Vec<u8> = Vec::from([255]);
        let expected: String = String::from("11111111");

        // act
        let actual = convert_bytes_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_bytes_to_binary_in_string_ok_2() {
        // arrange
        let input: Vec<u8> = Vec::from([0]);
        let expected: String = String::from("00000000");

        // act
        let actual = convert_bytes_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_bytes_to_binary_in_string_ok_3() {
        // arrange
        let input: Vec<u8> = Vec::from([0, 1]);
        let expected: String = String::from("0000000000000001");

        // act
        let actual = convert_bytes_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_bytes_to_binary_in_string_ok_4() {
        // arrange
        let input: Vec<u8> = Vec::from([0, 1, 2]);
        let expected: String = String::from("000000000000000100000010");

        // act
        let actual = convert_bytes_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn convert_bytes_to_binary_in_string_ok_5() {
        // arrange
        let input: Vec<u8> = Vec::from([0, 0, 255]);
        let expected: String = String::from("000000000000000011111111");

        // act
        let actual = convert_bytes_to_binary_in_string(input);

        // assert
        assert_eq!(expected, actual);
    }
}
