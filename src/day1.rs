
pub struct Input {
    lefts: [u32; 1000],
    rights: [u32; 1000],
}

#[aoc_runner_derive::aoc_generator(day1)]
pub fn input_generator(input: &[u8]) -> Input {

    let mut input = input;
    let mut result = Input { lefts: [0; 1000], rights: [0; 1000], };  
    // because input is predictible, we can go fast
    // (and panic of something goes wrong)

    for i in 0..999 {
        result.lefts[i] = crate::utils::parse_chars_to_u32(&input[0..5]);
        result.rights[i] = crate::utils::parse_chars_to_u32(&input[8..13]);
        input = &input[14..]; // 14 : 2 5 digits numbers, 3 spaces, one new line
    }
    result.lefts[999] = crate::utils::parse_chars_to_u32(&input[0..5]);
    result.rights[999] = crate::utils::parse_chars_to_u32(&input[8..13]);

    result
}

#[aoc_runner_derive::aoc(day1, part1, Sort)]
pub fn solve_part1(input: &Input) -> u32 {
    let mut lefts = input.lefts.clone();
    let mut rights = input.rights.clone();
    
    lefts.sort_unstable();
    rights.sort_unstable();
    
    lefts.into_iter().zip(rights.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

#[aoc_runner_derive::aoc(day1, part1, FlatArr)]
pub fn solve_part1_flatarr(input: &Input) -> u32 {
    let mut lefts = [0u32; 99999];
    let mut rights = [0u32; 99999];
    
    for left in input.lefts.iter() {
        lefts[*left as usize] += 1;
    }
    for right in input.rights.iter() {
        rights[*right as usize] += 1;
    }

    let mut left_index = 0;
    let mut right_index = 0;

    let mut result = 0;

    for _ in 0..1000 {
        while lefts[left_index] == 0 {
            left_index += 1;
        }
        while rights[right_index] == 0 {
            right_index += 1;
        }
        result += left_index.abs_diff(left_index) as u32;
    }

    result
}

#[aoc_runner_derive::aoc(day1, part1, BTreeMap)]
pub fn solve_part1_btreemap(input: &Input) -> u32 {
    let mut lefts = std::collections::BTreeMap::new();
    let mut rights = std::collections::BTreeMap::new();
    
    for left in input.lefts.iter() {
        match lefts.get_mut(left) {
            Some(occ) => *occ += 1,
            None => { lefts.insert(*left, 1); },
        }
    }
    for right in input.rights.iter() {
        match rights.get_mut(right) {
            Some(occ) => *occ += 1,
            None => { lefts.insert(*right, 1); },
        }
    }

    lefts.into_iter().map(|(val, occ)| std::iter::repeat(val).take(occ)).flatten()
        .zip(rights.into_iter().map(|(val, occ)| std::iter::repeat(val).take(occ)).flatten())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

#[aoc_runner_derive::aoc(day1, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    
    // precompute occurences in map
    let mut map = Box::new([0u32; 99999]);
    for right in input.rights.iter() {
        map[*right as usize] += 1;
    }

    input.lefts.iter().map(|left| *left * map[*left as usize]).sum()
}
