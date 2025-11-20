use crate::error::errors::AppError;

#[derive(Debug)]
pub struct PurchaseAmount {
    money: u32
}

impl PurchaseAmount {
    pub fn new(money: u32) -> Result<PurchaseAmount, AppError> {
        Self::validate(money)?;
        Ok(PurchaseAmount { money })
    }

    pub fn money(&self) -> u32 {
        self.money
    }

    fn validate(money: u32) -> Result<(), AppError> {
        if money < 1000 {
            return Err(AppError::MoneyTooSmall)
        }
        
        if money % 1000 != 0 {
            return Err(AppError::MoneyInvalidUnit)
        }

        Ok(())
    }
}