use strum_macros::EnumIter;

#[derive(Debug, Eq, PartialEq, Hash, EnumIter)]
pub enum LottoRank {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    FIFTH,
    NOTHING,
}

impl LottoRank {
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