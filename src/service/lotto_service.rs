use crate::domain::{BonusNumber, Lotto, PurchaseAmount, WinningNumbers};
use crate::error::AppError;
use crate::util::NumberGenerator;

pub struct LottoService;

impl LottoService {
    pub fn purchase(purchase_amount: &PurchaseAmount) -> Vec<Lotto> {
        let mut lottos: Vec<Lotto> = Vec::new();
        let purchase_count = purchase_amount.money() / 1000;

        for _ in 0..purchase_count {
            let lotto_numbers = NumberGenerator::generate_numbers_in_range(1, 45, 6);
            let lotto = match Lotto::new(lotto_numbers) {
                Ok(value) => value,
                Err(e) => panic!("{}", e.message()),
            };
            lottos.push(lotto);
        }

        lottos
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
        let purchase_amount = PurchaseAmount::new(5000).unwrap();

        // when
        let lottos = LottoService::purchase(&purchase_amount);

        // then
        assert_eq!(lottos.len(), 5);
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
