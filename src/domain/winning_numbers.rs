use std::collections::HashSet;

use crate::error::errors::DomainError;

#[derive(Debug)]
pub struct WinningNumbers {
    numbers: Vec<u32>
}

impl WinningNumbers {
    pub fn new(numbers: Vec<u32>) -> Result<WinningNumbers, DomainError> {
        Self::validate(&numbers)?;
        Ok(WinningNumbers { numbers })
    }

    pub fn numbers(&self) -> &[u32] {
        &self.numbers
    }

    fn validate(numbers: &[u32]) -> Result<(), DomainError> {
        if numbers.len() != 6 {
            return Err(DomainError::NumbersInvalidSize)
        }

        if numbers.len() != numbers.iter().collect::<HashSet<_>>().len() {
            return Err(DomainError::NumbersDuplicate)
        }

        if numbers.iter().any(|number| *number < 1 || *number > 45) {
            return Err(DomainError::NumbersInvalidRange)
        }
        
        Ok(())
    }
}