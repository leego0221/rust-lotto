use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::domain::bonus_number::BonusNumber;
use crate::domain::lotto::Lotto;
use crate::domain::lotto_rank::LottoRank;
use crate::domain::purchase_amount::PurchaseAmount;
use crate::domain::winning_numbers::WinningNumbers;

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
            // 이 부분에서 자동으로 참조자 단계를 맞춰주지만, 명시적으로 확인할 수 있도록 *number 적용
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