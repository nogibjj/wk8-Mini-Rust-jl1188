pub fn reverse_bits(input: u32) -> u32 {
    let mut result = 0u32;
    for i in 0..=31 {
        let bit_left = input >> i & 1;
        let bit_pos = 31 - i;
        result |= bit_left << (bit_pos);
    }
    result
}
