use std::cmp::min;


const ZERO_U8: u8 = '0' as u8;
const NINE_U8: u8 = '9' as u8;
const SPACE_U8: u8 = ' ' as u8;
const NEWLINE_U8: u8 = '\n' as u8;

#[inline(always)]
fn parse_num(input: &mut &[u8]) -> i8 {
    let first_digit = input[0] - ZERO_U8;
    let res = match input[1] {
        digit @ ZERO_U8..=NINE_U8 => {
            *input = &input[2..];
            first_digit * 10 + digit - ZERO_U8
        },
        SPACE_U8 | NEWLINE_U8 => {
            *input = &input[1..];
            first_digit
        },
        other => panic!(
            "Unexpected char: {}",
            char::from(unsafe { std::mem::transmute::<u8, u8>(other)}),
        ),
    };
    unsafe { std::mem::transmute(res) }
}

#[inline(always)]
fn parse_num2(input: &[u8]) -> i8 {
    let res = match input.len() {
        1 => input[0] - ZERO_U8,
        2 => (input[0] - ZERO_U8) * 10 + input[1] - ZERO_U8,
        other => panic!("Day 2 inputs should not contain numbers with {} digits!", other),
    };
    unsafe { std::mem::transmute(res) }
}

#[aoc_runner_derive::aoc(day2, part1)]
pub fn part1(input: &[u8]) -> u32 {
    let mut input: &[u8] = unsafe { std::mem::transmute(input) };

    let mut result = 0;

    for _ in 0..1000 {
        let mut a = parse_num(&mut input);
        input = &input[1..];
        let mut b = parse_num(&mut input);
        input = &input[1..];
        let mut c = parse_num(&mut input);

        let valid = loop {
            if ((a - b) * (b - c) <= 0) || (a.abs_diff(b) > 3) || (b.abs_diff(c) > 3) {
                while input[0] != NEWLINE_U8 {
                    input = &input[1..];
                }
                input = &input[1..];
                break false;
            }
            match input[..] {
                [] => break true,
                [NEWLINE_U8, .. ] => {
                    input = &input[1..];
                    break true
                },
                [SPACE_U8, .. ]=> {
                    input = &input[1..];
                    a = b;
                    b = c;
                    c = parse_num(&mut input);
                },
                [other, .. ] => panic!(
                    "Invalid input: unexpected char {}",
                    char::from(unsafe { std::mem::transmute::<u8, u8>(other) })
                ),
            }
        };

        result += valid as u32;
    }

    result
}

fn test_report(nums: &[i8], exclude_index: usize) -> bool {

    let iter = nums.iter()
        .enumerate()
        .filter(|(idx, _)| *idx != exclude_index)
        .map(|(_, value)| value)
        .cloned();

    let iter = crate::utils::window_iter::<_, _, 3>(iter);

    for [a, b, c] in iter {
        if (a - b) * (b - c) <= 0 || a.abs_diff(b) > 3 || b.abs_diff(c) > 3 {
            return false;
        }
    }

    true
}


pub fn part2_chars(input: &[u8]) -> u32 {

    let mut result = 0;

    let mut report = [0i8; 32];
    let mut report_length;

    for line in crate::utils::slice_iter::<{'\n' as u8}, 5>(input) {
        
        report_length = 0;

        for value in crate::utils::slice_iter::<{' ' as u8}, 1>(line) {
            report[report_length] = parse_num2(value);
            report_length += 1;
        }

        for maybe_exlude in 0..report_length+1 {
            if test_report(&report[0..report_length], maybe_exlude) {
                result += 1;
                break
            }
        }
    }

    result
}


#[aoc_runner_derive::aoc(day2, part2)]
pub fn part2(input: &[u8]) -> u32 {
    part2_chars(input)
}



