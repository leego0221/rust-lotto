use std::collections::HashSet;

use crate::error::AppError;

#[derive(Debug)]
pub struct WinningNumbers {
    numbers: Vec<u32>
}

impl WinningNumbers {
    pub fn new(numbers: Vec<u32>) -> Result<WinningNumbers, AppError> {
        Self::validate(&numbers)?;
        Ok(WinningNumbers { numbers })
    }

    pub fn numbers(&self) -> &[u32] {
        &self.numbers
    }

    fn validate(numbers: &[u32]) -> Result<(), AppError> {
        if numbers.len() != 6 {
            return Err(AppError::NumbersInvalidSize)
        }

        if numbers.len() != numbers.iter().collect::<HashSet<_>>().len() {
            return Err(AppError::NumbersDuplicate)
        }

        if numbers.iter().any(|number| *number < 1 || *number > 45) {
            return Err(AppError::NumbersInvalidRange)
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod winning_numbers_tests {
    use super::*;

    #[test]
    fn valid_winning_numbers() {
        // given
        let numbers = vec![1, 2, 3, 4, 5, 6];

        // when
        let result = WinningNumbers::new(numbers);

        // then
        assert_eq!(result.unwrap().numbers(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn numbers_invalid_size() {
        // given
        let numbers = vec![1, 2, 3, 4, 5];

        // when
        let result = WinningNumbers::new(numbers);

        // then
        assert_eq!(result.unwrap_err(), AppError::NumbersInvalidSize);
    }

    #[test]
    fn numbers_duplicate() {
        // given
        let numbers = vec![1, 2, 3, 4, 5, 5];

        // when
        let result = WinningNumbers::new(numbers);

        // then
        assert_eq!(result.unwrap_err(), AppError::NumbersDuplicate);
    }

    #[test]
    fn numbers_invalid_range() {
        // given
        let numbers = vec![1, 2, 3, 4, 5, 100];

        // when
        let result = WinningNumbers::new(numbers);

        // then
        assert_eq!(result.unwrap_err(), AppError::NumbersInvalidRange);
    }
}
