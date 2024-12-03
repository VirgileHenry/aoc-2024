use std::{collections::BTreeSet, fmt::Display};

use rayon::iter::IntoParallelIterator;



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

struct SplitIter<'a, const SPLIT: u8, const MIN_LINE_SIZE: usize> {
    rest: &'a [u8],
}

impl<'a, const SPLIT: u8, const MIN_LINE_SIZE: usize> Iterator for SplitIter<'a, SPLIT, MIN_LINE_SIZE> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.len() < MIN_LINE_SIZE {
            None
        }
        else {
            let result = self.rest;
            self.rest = &self.rest[MIN_LINE_SIZE..];
            for (index, val) in self.rest.iter().enumerate() {
                if *val == SPLIT {
                    self.rest = &self.rest[(index + 1)..];
                    return Some(&result[..(MIN_LINE_SIZE + index)])
                }
            }
            self.rest = &[];
            Some(result)
        }
    }
}

pub fn slice_iter<const SPLIT: u8, const MIN_LINE_SIZE: usize>(input: &[u8]) -> impl Iterator<Item = &[u8]> {
    assert_ne!(MIN_LINE_SIZE, 0, "Slice iter with zero min size in not supported");
    SplitIter::<'_, SPLIT, MIN_LINE_SIZE> {
        rest: input
    }
}


struct WindowIter<T: Copy + Default, I: Iterator<Item = T>, const SIZE: usize> {
    buffer: [T; SIZE],
    iter: I,
}

impl<T: Copy + Default, I: Iterator<Item = T>, const SIZE: usize> Iterator for WindowIter<T, I, SIZE> {
    type Item = [T; SIZE];
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..SIZE-1 {
            self.buffer[i] = self.buffer[i+1];
        }
        self.buffer[SIZE - 1] = self.iter.next()?;
        Some(self.buffer)
    }
}

pub fn window_iter<T: Copy + Default, I: Iterator<Item = T>, const SIZE: usize>(iter: I) -> impl Iterator<Item = [T; SIZE]> {
    let mut buffer = [T::default(); SIZE];
    let mut iter = iter;
    for i in 0..SIZE-1 {
        match iter.next() {
            Some(elem) => buffer[i+1] = elem,
            None => {},
        }
    }
    WindowIter::<T, I, SIZE> {
        buffer,
        iter,
    }
}


pub fn parallelize<D: Display + std::ops::Add<Output = D> + Send + std::iter::Sum, F: Fn(&[u8]) -> D + Send + Sync>(input: &[u8], f: F) -> D {
    
    use rayon::iter::ParallelIterator;

    let split_count = num_cpus::get();

    let indexes = std::iter::once(0)
        .chain((1..(split_count-1)).map(|index| {
            let mut attempt_index = index * input.len() / split_count;
            loop {
                match input.get(attempt_index) {
                    Some(10) | None => break, // 10 is '\n' as u8
                    _ => attempt_index += 1,
                } 
            }
            attempt_index + 1
        }))
        .chain(std::iter::once(input.len()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    println!("splits: {indexes:?}");

    let inputs = indexes.windows(2)
        .map(|window| &input[window[0]..window[1]])
        .collect::<Vec<_>>();

    inputs.into_par_iter().map(f).sum()

}


#[cfg(test)]
mod test {
    use super::slice_iter;


    #[test]
    fn test_slice_iter() {
        
        let mut iter = slice_iter::<{' ' as u8}, 1>("1 2 3 4".as_bytes());
        assert_eq!(iter.next(), Some("1".as_bytes()));
        assert_eq!(iter.next(), Some("2".as_bytes()));
        assert_eq!(iter.next(), Some("3".as_bytes()));
        assert_eq!(iter.next(), Some("4".as_bytes()));
        assert_eq!(iter.next(), None);
        
        let mut iter = slice_iter::<{' ' as u8}, 2>("1 2 3 4".as_bytes());
        assert_eq!(iter.next(), Some("1 2".as_bytes()));
        assert_eq!(iter.next(), Some("3 4".as_bytes()));
        assert_eq!(iter.next(), None);
        
        let mut iter = slice_iter::<{' ' as u8}, 3>("1 2 3 4".as_bytes());
        assert_eq!(iter.next(), Some("1 2".as_bytes()));
        assert_eq!(iter.next(), Some("3 4".as_bytes()));
        assert_eq!(iter.next(), None);
        
        let mut iter = slice_iter::<{' ' as u8}, 4>("1 2 3 4".as_bytes());
        assert_eq!(iter.next(), Some("1 2 3".as_bytes()));
        assert_eq!(iter.next(), None);
    }



}