use std::collections::HashSet;

use crate::error::errors::AppError;

#[derive(Debug)]
pub struct Lotto {
    numbers: Vec<u32>
}

impl Lotto {
    pub fn new(numbers: Vec<u32>) -> Result<Lotto, AppError> {
        Self::validate(&numbers)?;
        Ok(Lotto { numbers })
    }

    pub fn numbers(&self) -> &[u32] {
        &self.numbers
    }

    fn validate(numbers: &[u32]) -> Result<(), AppError> {
        if numbers.len() != 6 {
            return Err(AppError::LottoInvalidSize)
        }

        if numbers.len() != numbers.iter().collect::<HashSet<_>>().len() {
            return Err(AppError::LottoDuplicate)
        }

        if numbers.iter().any(|number| *number < 1 || *number > 45) {
            return Err(AppError::LottoInvalidRange)
        }
        
        Ok(())
    }
}
