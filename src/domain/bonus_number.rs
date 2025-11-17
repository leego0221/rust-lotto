use crate::error::errors::DomainError;

#[derive(Debug)]
pub struct BonusNumber {
    number: u32
}

impl BonusNumber {
    pub fn new(number: u32) -> Result<BonusNumber, DomainError> {
        Self::validate(number)?;
        Ok(BonusNumber { number })
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    fn validate(number: u32) -> Result<(), DomainError> {
        if number < 1 || number > 45 {
            return Err(DomainError::BonusInvalidRange)
        }

        Ok(())
    }
}