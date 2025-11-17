use rand::seq::SliceRandom;

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
