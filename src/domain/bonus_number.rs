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

#[cfg(test)]
mod bonus_number_tests {
    use super::*;

    #[test]
    fn valid_bonus_number() {
        // given
        let number = 7;

        // when
        let result = BonusNumber::new(number);
        
        // then
        assert_eq!(result.unwrap().number(), 7);
    }

    #[test]
    fn bonus_invalid_range() {
        // given
        let number = 100;

        // when
        let result = BonusNumber::new(number);

        // then
        assert_eq!(result.unwrap_err(), AppError::BonusInvalidRange);
    }
}
