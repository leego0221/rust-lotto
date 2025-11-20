use crate::domain::purchase_amount::PurchaseAmount;
use crate::domain::winning_numbers::WinningNumbers;
use crate::domain::bonus_number::BonusNumber;
use crate::domain::lotto::Lotto;
use crate::error::errors::AppError;
use crate::util::number_generator;

pub fn purchase(purchase_amount: &PurchaseAmount) -> Vec<Lotto> {
    let mut lottos: Vec<Lotto> = Vec::new();
    let purchase_count = purchase_amount.money() / 1000;

    for _ in 0..purchase_count {
        let lotto_numbers = number_generator::generate_numbers_in_range(1, 45, 6);
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