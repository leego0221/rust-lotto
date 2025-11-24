use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::domain::{BonusNumber, Lotto, LottoRank, PurchaseAmount, WinningNumbers};

pub struct LottoRankService {
    rank_counter: HashMap<LottoRank, i32>,
}

impl LottoRankService {
    pub fn new() -> Self {
        let rank_counter: HashMap<LottoRank, i32> = LottoRank::iter()
            .map(|rank| (rank, 0))
            .collect();

        LottoRankService { rank_counter }
    }

    pub fn determine_ranks(&mut self, lottos: &[Lotto], winning_numbers: &WinningNumbers, bonus_number: &BonusNumber) {
        lottos.iter().for_each(|lotto| {
            let winning_numbers_count = Self::count_winning_numbers(lotto, winning_numbers);
            let bonus_number_matched = Self::bonus_number_matched(lotto, bonus_number);

            let rank = LottoRank::get_rank(winning_numbers_count, bonus_number_matched);
            self.update_rank(rank);
        });
    }

    pub fn get_rank(&self) -> &HashMap<LottoRank, i32> {
        &self.rank_counter
    }

    pub fn calculate_profit_rate(&self, purchase_amount: &PurchaseAmount) -> f64 {
        let total_prize: u64 = self.rank_counter.iter()
            .map(|(rank, count)| rank.get_prize() as u64 * *count as u64)
            .sum();

        total_prize as f64 / purchase_amount.money() as f64 * 100.0
    }

    fn count_winning_numbers(lotto: &Lotto, winning_numbers: &WinningNumbers) -> usize {
        winning_numbers.numbers()
            .iter()
            .filter(|number| lotto.numbers().contains(*number))
            .count()
    }

    fn bonus_number_matched(lotto: &Lotto, bonus_number: &BonusNumber) -> bool {
        lotto.numbers().contains(&bonus_number.number())
    }

    fn update_rank(&mut self, rank: LottoRank) {
        let count = self.rank_counter
            .get(&rank)
            .copied()
            .unwrap_or(0);
        
        self.rank_counter.insert(rank, count + 1);
    }
}

#[cfg(test)]
mod lotto_rank_service_tests {
    use super::*;

    #[test]
    fn determine_ranks_test() {
        // given
        let mut lotto_rank_service = LottoRankService::new();

        let lottos = vec![
            Lotto::new(vec![1, 2, 3, 4, 5, 6]).unwrap(),
            Lotto::new(vec![7, 8, 9, 10, 11, 12]).unwrap(),
        ];
        let winning_numbers = WinningNumbers::new(vec![1, 2, 3, 4, 6, 7]).unwrap();
        let bonus_number = BonusNumber::new(5).unwrap(); // 2등

        // when
        lotto_rank_service.determine_ranks(&lottos, &winning_numbers, &bonus_number);
        let result = lotto_rank_service.get_rank();

        // then
        assert_eq!(*result.get(&LottoRank::SECOND).unwrap(), 1);
    }

    #[test]
    fn calculate_profit_rate_test() {
        // given
        let mut lotto_rank_service = LottoRankService::new();

        let purchase_amount = PurchaseAmount::new(1000).unwrap(); // 1개 구매
        let lottos = vec![Lotto::new(vec![1, 2, 3, 4, 5, 6]).unwrap()]; // 1000원
        let winning_numbers = WinningNumbers::new(vec![1, 2, 3, 7, 8, 9]).unwrap(); // 4등: 5000원
        let bonus_number = BonusNumber::new(5).unwrap();

        lotto_rank_service.determine_ranks(&lottos, &winning_numbers, &bonus_number);

        // when
        let result = lotto_rank_service.calculate_profit_rate(&purchase_amount);

        // then
        assert_eq!(result, 500 as f64);
    }
}
