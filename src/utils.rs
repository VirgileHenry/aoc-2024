

/// Sweet little function to quickly parse AOC number inputs
/// As most of the times, the numbers have fixed size, we can assume a correct slice
/// reject Rust safety, embrace C speed
pub fn parse_chars_to_u32(input: &[u8]) -> u32 {
    const OFFSET: u32 = '0' as u32;
    let mut result = 0;
    for ch in input {
        result = result * 10 + (*ch as u32 - OFFSET);
    };
    result
}