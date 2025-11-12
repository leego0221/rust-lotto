#[derive(Debug)]
pub struct Lotto {
    numbers: Vec<u32>
}

impl Lotto {
    pub fn new(numbers: Vec<u32>) -> Lotto {
        Lotto { numbers }
    }

    pub fn get_numbers(&self) -> Vec<u32> {
        self.numbers.clone()
    }
}