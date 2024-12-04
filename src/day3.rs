

pub fn part1(input: &str) -> impl std::fmt::Display {
    part1_rabbit(input.as_bytes())
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    part2_rabbit(input.as_bytes())
}

#[aoc_runner_derive::aoc(day3, part1, Regex)]
pub fn part1_regex(input: &str) -> u32 {
    regex::Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|capture| {
            capture.get(1).unwrap().as_str().parse::<u32>().unwrap() * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}


#[aoc_runner_derive::aoc(day3, part2, Regex)]
pub fn part2_regex(input: &str) -> u32 {
    let mut do_state= true;
    regex::Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))")
        .unwrap()
        .captures_iter(input)
        .map(|capture| {
            match (do_state, capture.get(0).unwrap().as_str()) {
                (true, "don't()") => {do_state = false; 0},
                (true, "do()") => {do_state = true; 0},
                (false, "do()") => {do_state = true; 0},
                (true, _) => capture.get(1).unwrap().as_str().parse::<u32>().unwrap() * capture.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                _ => 0,
            }
        })
        .sum()
}

const MULP: u32 = (('m' as u32) << 24) + (('u' as u32) << 16) + (('l' as u32) << 8) + (('(' as u32) << 0);
const MULTP_LEN: usize = 8;

const JUMP_TABLE_P1: [usize; 256] = [
//  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 4, 0, 8, 8, 0, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 5, 7, 8, 8, 8, 8, 8, 8, 8, 6, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
];

#[aoc_runner_derive::aoc(day3, part1, Rabbit)]
pub fn part1_rabbit(input: &[u8]) -> u32 {
    
    let mut result = 0;
    let mut input = input;

    while input.len() > 7 {
        let next_jump = JUMP_TABLE_P1[input[MULTP_LEN-1] as usize];
        if next_jump > 0 {
            input = &input[next_jump..];
        }
        else
        {
            let start = ((input[0] as u32) << 24) +  ((input[1] as u32) << 16) +  ((input[2] as u32) << 8) +  ((input[3] as u32) << 0);
            if start != MULP {
                input = &input[1..];
                continue;
            }
            input = &input[4..];

            let mut d1: u32 = 0;
            while '0' as u8 <= input[0] && input[0] <= '9' as u8 {
                d1 = d1 * 10 + (input[0] - '0' as u8) as u32;
                input = &input[1..];
            }
            
            if input[0] != ',' as u8 {
                input = &input[1..];
                continue;
            }
            input = &input[1..];
            
            let mut d2: u32 = 0;
            while '0' as u8 <= input[0] && input[0] <= '9' as u8 {
                d2 = d2 * 10 + (input[0] - '0' as u8) as u32;
                input = &input[1..]
            }
            
            if input[0] != ')' as u8 {
                input = &input[1..];
                continue;
            }
            input = &input[1..];
            
            result += d1 * d2;
        }
    }

    result
}

const DOPP: u32 = (('d' as u32) << 24) + (('o' as u32) << 16) + (('(' as u32) << 8) + ((')' as u32) << 0);
const DONTPP_B: u32 = (('d' as u32) << 24) + (('o' as u32) << 16) + (('n' as u32) << 8) + (('\'' as u32) << 0);
const DONTPP_S: u32 = (('t' as u32) << 16) + (('(' as u32) << 8) + ((')' as u32) << 0);

const DO_LEN: usize = 4;
const DONT_LEN: usize = 7;

const JUMP_TABLE_P2: [usize; 512] = [
//  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 3, 1, 0, 7, 7, 0, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 6, 7, 7, 7, 7, 7, 7, 7, 4, 6, 4, 5, 7, 7, 7, 7, 2, 5, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
//  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 1, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];

#[aoc_runner_derive::aoc(day3, part2, Rabbit)]
pub fn part2_rabbit(input: &[u8]) -> u32 {
    
    let mut result = 0;
    let mut input = input;
    let mut table_offset = 0;
    let mut count_muls = true;
    let mut fetch_length = DONT_LEN;

    while input.len() > 7 {

        let next_jump = JUMP_TABLE_P2[input[fetch_length-1] as usize + table_offset];
        if next_jump > 0 {
            input = &input[next_jump..];
        }
        else
        {
            let start_b = ((input[0] as u32) << 24) + ((input[1] as u32) << 16) + ((input[2] as u32) << 8) + ((input[3] as u32) << 0);
            let start_s = ((input[4] as u32) << 16) +  ((input[5] as u32) << 8) + ((input[6] as u32) << 0);

            match (count_muls, start_b, start_s) {
                (true, DONTPP_B, DONTPP_S) => {
                    table_offset = 256;
                    input = &input[fetch_length..];
                    fetch_length = DO_LEN;
                    count_muls = false;
                    continue;
                },
                (false, DOPP, _) => {
                    table_offset = 0;
                    input = &input[fetch_length..];
                    fetch_length = DONT_LEN;
                    count_muls = true;
                    continue;
                },
                (true, MULP, _) => {
                    input = &input[4..];
                },
                _ => {
                    input = &input[1..];
                    continue;
                }
            }

            let mut d1: u32 = 0;
            while '0' as u8 <= input[0] && input[0] <= '9' as u8 {
                d1 = d1 * 10 + (input[0] - '0' as u8) as u32;
                input = &input[1..];
            }
                
            if input[0] != ',' as u8 {
                input = &input[1..];
                continue;
            }
            input = &input[1..];
                
            let mut d2: u32 = 0;
            while '0' as u8 <= input[0] && input[0] <= '9' as u8 {
                d2 = d2 * 10 + (input[0] - '0' as u8) as u32;
                input = &input[1..]
            }
                
            if input[0] != ')' as u8 {
                input = &input[1..];
                continue;
            }
            input = &input[1..];
                
            result += d1 * d2;
        }
    }

    result
}

