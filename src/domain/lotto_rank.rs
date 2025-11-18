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