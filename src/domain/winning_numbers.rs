#[derive(Debug)]
pub struct WinningNumbers {
    numbers: Vec<u32>
}

impl WinningNumbers {
    pub fn new(numbers: Vec<u32>) -> WinningNumbers {
        WinningNumbers { numbers }
    }

    pub fn get_numbers(&self) -> Vec<u32> {
        self.numbers.clone()
    }
}