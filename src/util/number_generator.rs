use rand::seq::SliceRandom;

pub struct NumberGenerator;

impl NumberGenerator {
    pub fn generate_numbers_in_range(start: usize, end: usize, count: usize) -> Vec<u32> {
        let mut all_numbers = Vec::new();
        for number in start..=end {
            all_numbers.push(number as u32);
        }

        let mut rng = rand::rng();
        all_numbers.shuffle(&mut rng);

        let numbers = &all_numbers[0..count];
        numbers.to_vec()
    }
}

#[cfg(test)]
mod number_generator_tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn generate_numbers_in_range_test() {
        // given
        let start = 1;
        let end = 45;
        let count = 6;

        // when
        let numbers = NumberGenerator::generate_numbers_in_range(start, end, count);

        // then
        assert!(numbers.iter().all(|n| *n >= 1 && *n <= 45));
        assert_eq!(numbers.len(), 6);
        assert_eq!(numbers.iter().collect::<HashSet<_>>().len(), 6);
    }
}
