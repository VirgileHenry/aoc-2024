

pub fn part1(input: &str) -> impl std::fmt::Display {
    part1_scan(input.as_bytes())
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    part2_scan(input.as_bytes())
}


const LS: i32 = 141;
const LC: i32 = 140;

#[aoc_runner_derive::aoc(day4, part1, Naive)]
pub fn part1_naive(input: &[u8]) -> u32 {

    const DIRECTIONS_P1: &[i32] = &[
        -LS - 1, -LS, -LS + 1, -1, 1, LS - 1, LS, LS + 1,
    ];

    let mut result = 0;

    for (index, val) in input.iter().enumerate() {
        if *val == ('X' as u8) {
            for direction in DIRECTIONS_P1.iter() {
                if input.get((index as i32 + direction) as usize).cloned() != Some('M' as u8) {
                    continue;
                }
                if input.get((index as i32 + direction * 2) as usize).cloned() != Some('A' as u8) {
                    continue;
                }
                if input.get((index as i32 + direction * 3) as usize).cloned() != Some('S' as u8) {
                    continue;
                }
                result += 1;
            }
        }
    }

    result
}


#[aoc_runner_derive::aoc(day4, part1, Scan)]
pub fn part1_scan(input: &[u8]) -> u32 {

    const DIRECTIONS_P1_SCAN: [usize; 4] = [
        1, LS as usize - 1, LS as usize, LS as usize + 1,
    ];
    const XMAS: u32 = 0b01011000_01001101_01000001_01010011;
    const SMAX: u32 = 0b01010011_01000001_01001101_01011000;

    let mut result = 0;

    for direction in DIRECTIONS_P1_SCAN {
        for i in 0..((LS*LC-1) as usize - 3*direction) {
            let word =
                (((input[i + 0 * direction as usize] as u32) << 24) | ((input[i + 1 * direction as usize] as u32) << 16)) |
                (((input[i + 2 * direction as usize] as u32) << 8) | ((input[i + 3 * direction as usize] as u32) << 0));
            result += (word == XMAS) as u32 + (word == SMAX) as u32;
        }
    }

    result
}


#[aoc_runner_derive::aoc(day4, part2, Naive)]
pub fn part2_naive(input: &[u8]) -> u32 {

    const DIRECTIONS_P2: &[(i32, i32)] = &[
        (-LS - 1, LS + 1),
        (-LS + 1, LS - 1),
        (LS - 1, -LS + 1),
        (LS + 1, -LS - 1),
    ];

    let mut result = 0;

    for (index, val) in input.iter().enumerate() {
        if *val == ('A' as u8) {
            let mut valid_diag = 0;
            for (m_dir, s_dir) in DIRECTIONS_P2.iter() {
                if input.get((index as i32 + m_dir) as usize).cloned() != Some('M' as u8) {
                    continue;
                }
                if input.get((index as i32 + s_dir) as usize).cloned() != Some('S' as u8) {
                    continue;
                }
                valid_diag += 1;
            }
            if valid_diag == 2 {
                result += 1;
                continue;
            }
        }
    }

    result
}


#[aoc_runner_derive::aoc(day4, part2, Scan)]
pub fn part2_scan(input: &[u8]) -> u32 {

    const START: usize = (LS+1) as usize;
    const END: usize = ((LS * LC) - (LS+1) - 1) as usize;

    let mut result = 0;

    for index in START..END {
        result += unsafe { ((
            !input.get_unchecked(index) &
            (input.get_unchecked((index as i32-LS-1) as usize) ^ input.get_unchecked((index as i32+LS+1) as usize)) &
            (input.get_unchecked((index as i32-LS+1) as usize) ^ input.get_unchecked((index as i32+LS-1) as usize))
        ) == 0b00011110) as u32 }
    }

    result
}


