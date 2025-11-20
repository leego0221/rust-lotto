use std::collections::HashSet;

use crate::error::AppError;

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

#[cfg(test)]
mod lotto_tests {
    use super::*;

    #[test]
    fn valid_lotto() {
        // given
        let numbers = vec![1, 2, 3, 4, 5, 6];

        // when
        let result = Lotto::new(numbers);

        // then
        assert_eq!(result.unwrap().numbers(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn lotto_invalid_size() {
        // given
        let numbers = vec![1, 2, 3, 4, 5];

        // when
        let result = Lotto::new(numbers);

        // then
        assert_eq!(result.unwrap_err(), AppError::LottoInvalidSize);
    }

    #[test]
    fn lotto_duplicate() {
        // given
        let numbers = vec![1, 2, 3, 4, 5, 5];

        // when
        let result = Lotto::new(numbers);

        // then
        assert_eq!(result.unwrap_err(), AppError::LottoDuplicate);
    }

    #[test]
    fn lotto_invalid_range() {
        // given
        let numbers = vec![1, 2, 3, 4, 5, 100];

        // when
        let result = Lotto::new(numbers);

        // then
        assert_eq!(result.unwrap_err(), AppError::LottoInvalidRange);
    }
}
