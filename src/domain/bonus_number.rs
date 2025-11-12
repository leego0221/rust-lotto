#[derive(Debug)]
pub struct BonusNumber {
    number: u32
}

impl BonusNumber {
    pub fn new(number: u32) -> BonusNumber {
        BonusNumber { number }
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }
}