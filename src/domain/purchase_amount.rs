use crate::error::AppError;

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

#[cfg(test)]
mod purchase_amount_tests {
    use super::*;

    #[test]
    fn valid_purchase_amount() {
        // given
        let money = 1000;

        // when
        let result = PurchaseAmount::new(money);

        // then
        assert_eq!(result.unwrap().money(), 1000);
    }

    #[test]
    fn money_too_small() {
        // given
        let money = 500;

        // when
        let result = PurchaseAmount::new(money);

        // then
        assert_eq!(result.unwrap_err(), AppError::MoneyTooSmall);
    }

    #[test]
    fn money_invalid_unit() {
        // given
        let money = 1500;

        // when
        let result = PurchaseAmount::new(money);

        // then
        assert_eq!(result.unwrap_err(), AppError::MoneyInvalidUnit);
    }
}
