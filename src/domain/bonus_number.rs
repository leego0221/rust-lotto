use crate::error::AppError;

#[derive(Debug)]
pub struct BonusNumber {
    number: u32
}

impl BonusNumber {
    pub fn new(number: u32) -> Result<BonusNumber, AppError> {
        Self::validate(number)?;
        Ok(BonusNumber { number })
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    fn validate(number: u32) -> Result<(), AppError> {
        if number < 1 || number > 45 {
            return Err(AppError::BonusInvalidRange)
        }

        Ok(())
    }
}
