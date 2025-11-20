use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum LottoRank {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    NOTHING,
}

impl LottoRank {
    pub fn get_rank(count: usize, matched: bool) -> LottoRank {
        match (count, matched) {
            (6, _) => LottoRank::FIRST,
            (5, true) => LottoRank::SECOND,
            (5, false) => LottoRank::THIRD,
            (4, _) => LottoRank::FOURTH,
            (3, _) => LottoRank::FIFTH,
            _ => LottoRank::NOTHING,
        }
    }

    pub fn get_prize(&self) -> u32 {
        match self {
            LottoRank::FIRST => 2_000_000_000,
            LottoRank::SECOND => 30_000_000,
            LottoRank::THIRD => 1_500_000,
            LottoRank::FOURTH => 50_000,
            LottoRank::FIFTH => 5_000,
            LottoRank::NOTHING => 0,
        }
    }
}

#[cfg(test)]
mod lotto_rank_tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn get_rank_test() {
        // given
        let count_and_matched = vec![
            (6, false), // 1등
            (5, true),  // 2등
            (5, false), // 3등
            (4, true),  // 4등
            (3, false), // 5등
            (2, true),  // 낙첨
        ];
        
        // when
        let result: Vec<LottoRank> = count_and_matched.iter()
            .map(|(count, matched)| LottoRank::get_rank(*count, *matched))
            .collect();

        // then
        assert_eq!(result, vec![
            LottoRank::FIRST,
            LottoRank::SECOND,
            LottoRank::THIRD,
            LottoRank::FOURTH,
            LottoRank::FIFTH,
            LottoRank::NOTHING
        ]);
    }

    #[test]
    fn get_prize_test() {
        // given and when
        let result: Vec<u32> = LottoRank::iter()
            .map(|rank| rank.get_prize())
            .collect();

        // then
        assert_eq!(result, vec![
            2_000_000_000,
            30_000_000,
            1_500_000,
            50_000,
            5_000,
            0,
        ]);
    }
}
