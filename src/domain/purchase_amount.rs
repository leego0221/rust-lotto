#[derive(Debug)]
pub struct PurchaseAmount {
    money: u32
}

impl PurchaseAmount {
    pub fn new(money: u32) -> PurchaseAmount {
        PurchaseAmount { money }
    }

    pub fn get_money(&self) -> u32 {
        self.money
    }
}