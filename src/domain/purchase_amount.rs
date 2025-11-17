use crate::error::errors::DomainError;

#[derive(Debug)]
pub struct PurchaseAmount {
    money: u32
}

impl PurchaseAmount {
    pub fn new(money: u32) -> Result<PurchaseAmount, DomainError> {
        let money = Self::validate(money)?;
        Ok(PurchaseAmount { money })
    }

    pub fn money(&self) -> u32 {
        self.money
    }

    fn validate(money: u32) -> Result<u32, DomainError> {
        if money < 1000 {
            return Err(DomainError::MoneyTooSmall)
        }
        if money % 1000 != 0 {
            return Err(DomainError::MoneyInvalidUnit)
        }

        Ok(money)
    }
}