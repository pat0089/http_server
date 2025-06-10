use std::vec;

/// Encode a string of characters to a vector of booleans representing the 0s and 1s of the units of length for either 
/// 
/// NOTE: input must be a valid ASCII 127 string
pub fn encode_to_barcode_bitstring(input: &str) -> Vec<bool> {
    assert!(input.is_ascii(), "Input must be an ASCII string");

    //map characters
    let mapped_characters = input.chars()
        .map(|c| CHARACTER_MAP_CODE128[c as usize - 32])
        .collect::<Vec<u16>>();
    
    //mapped characters into contiguous bitstring
    let mapped_in_binary = mapped_characters.iter()
        .map(|num| first_11_unsigned_16_to_bitstring(*num))
        .flatten()
        .collect::<Vec<bool>>();

    //quiet zone
    let quiet_zone = vec![false; 10];

    //start and stop code bitstrings
    let start_code = first_11_unsigned_16_to_bitstring(CHARACTER_MAP_CODE128[START_CODE_B]);
    let stop_code = first_11_unsigned_16_to_bitstring(CHARACTER_MAP_CODE128[STOP_CODE]);

    //checksum bitstring
    let checksum = get_checksum(input, START_CODE_B as u32);
    let checksum_code = first_11_unsigned_16_to_bitstring(CHARACTER_MAP_CODE128[checksum as usize]);

    //bringing it all together
    vec![quiet_zone.clone(), start_code, mapped_in_binary, checksum_code, stop_code, quiet_zone].into_iter().flatten().collect()
}

/// Convert a u16 to a bitstring of length 11
fn first_11_unsigned_16_to_bitstring(num: u16) -> Vec<bool> {
    assert!(num.leading_zeros() >=  5);
    let mut result = vec![];
    for i in 0..11 {
        result.push(num >> i & 1 == 1);
    }
    result.reverse();
    result
}

/// Calculate the checksum for a given string and start code
fn get_checksum(input: &str, start_code: u32) -> u32{
    let mut checksum = input.chars().enumerate()
        .map(|(i, c)| (c as u32 - 32) * (i + 1) as u32)
        .sum::<u32>();
    checksum += start_code;
    checksum % 103
}

const STOP_CODE: usize = 106;

//const START_CODE_A: usize = 103;
const START_CODE_B: usize = 104;
//const START_CODE_C: usize = 105;

const CHARACTER_MAP_CODE128: [u16; 107] = [
    0b11011001100,
    0b11001101100,
    0b11001100110,
    0b10010011000,
    0b10010001100,
    0b10001001100,
    0b10011001000,
    0b10011000100,
    0b10001100100,
    0b11001001000,
    0b11000100100,
    0b11001000100,
    0b10110011100,
    0b10011011100,
    0b10011001110,
    0b10111001100,
    0b10011101100,
    0b10011100110,
    0b11001110010,
    0b11001011100,
    0b11001001110,
    0b11011100100,
    0b11001110100,
    0b11101101110,
    0b11101001100,
    0b11100101100,
    0b11100100110,
    0b11101100100,
    0b11100110100,
    0b11100110010,
    0b11011011000,
    0b11011000110,
    0b11000110110,
    0b10100011000,
    0b10001011000,
    0b10001000110,
    0b10110001000,
    0b10001101000,
    0b10001100010,
    0b11010001000,
    0b11000101000,
    0b11000100010,
    0b10110111000,
    0b10110001110,
    0b10001101110,
    0b10111011000,
    0b10111000110,
    0b10001110110,
    0b11101110110,
    0b11010001110,
    0b11000101110,
    0b11011101000,
    0b11011100010,
    0b11011101110,
    0b11101011000,
    0b11101000110,
    0b11100010110,
    0b11101101000,
    0b11101100010,
    0b11100011010,
    0b11101111010,
    0b11001000010,
    0b11110001010,
    0b10100110000,
    0b10100001100,
    0b10010110000,
    0b10010000110,
    0b10000101100,
    0b10000100110,
    0b10110010000,
    0b10110000100,
    0b10011010000,
    0b10011000010,
    0b10000110100,
    0b10000110010,
    0b11000010010,
    0b11001010000,
    0b11110111010,
    0b11000010100,
    0b10001111010,
    0b10100111100,
    0b10010111100,
    0b10010011110,
    0b10111100100,
    0b10011110100,
    0b10011110010,
    0b11110100100,
    0b11110010100,
    0b11110010010,
    0b11011011110,
    0b11011110110,
    0b11110110110,
    0b10101111000,
    0b10100011110,
    0b10001011110,
    0b10111101000,
    0b10111100010,
    0b11110101000,
    0b11110100010,
    0b10111011110,
    0b10111101110,
    0b11101011110,
    0b11110101110,
    0b11010000100,
    0b11010010000,
    0b11010011100,
    0b11000111010
];


#[test]
fn test_encode_to_barcode_bitstring() {
    let test: u16 = 0b10011110000; //11 bit number to use as bitstring
    assert!(test.leading_zeros() >=  5);
    let test_result = first_11_unsigned_16_to_bitstring(test);
    println!("{:?}", test_result);

    let test_str = "PJJ123C";
    let checksum = get_checksum(test_str, START_CODE_B as u32 - 1);
    assert_eq!(checksum, 54);
}

