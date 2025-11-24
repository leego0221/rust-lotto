use crate::domain::{BonusNumber, Lotto, WinningNumbers};
use crate::error::AppError;

pub struct LottoService;

impl LottoService {
    pub fn purchase(lotto_numbers: Vec<u32>) -> Result<Lotto, AppError> {
        Ok(Lotto::new(lotto_numbers)?)
    }

    pub fn check_duplicate(winning_numbers: &WinningNumbers, bonus_number: &BonusNumber) -> Result<(), AppError> {
        if winning_numbers.numbers().iter().any(|number| *number == bonus_number.number()) {
            return Err(AppError::BonusDuplicatesWithNumbers)
        }

        Ok(())
    }
}

#[cfg(test)]
mod lotto_service_tests {
    use super::*;

    #[test]
    fn purchase_test() {
        // given
        let lotto_numbers = vec![1, 2, 3, 4, 5, 6];

        // when
        let result = LottoService::purchase(lotto_numbers);

        // then
        assert_eq!(result.unwrap().numbers(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn check_duplicate_test() {
        // given
        let winning_numbers = WinningNumbers::new(vec![1, 2, 3, 4, 5, 6]).unwrap();
        let bonus_number = BonusNumber::new(7).unwrap();

        // when
        let result = LottoService::check_duplicate(&winning_numbers, &bonus_number);

        // then
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn bonus_duplicates_with_numbers() {
        // given
        let winning_numbers = WinningNumbers::new(vec![1, 2, 3, 4, 5, 6]).unwrap();
        let bonus_number = BonusNumber::new(5).unwrap();

        // when
        let result = LottoService::check_duplicate(&winning_numbers, &bonus_number);

        // then
        assert_eq!(result.unwrap_err(), AppError::BonusDuplicatesWithNumbers);
    }
}
