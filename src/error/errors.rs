#[derive(Debug, PartialEq)]
pub enum AppError {
    InputEmpty,
    InputNotPositive,
    InputNotCharacter,

    MoneyTooSmall,
    MoneyInvalidUnit,

    SelectionInvalidMode,

    NumbersInvalidSize,
    NumbersDuplicate,
    NumbersInvalidRange,

    BonusInvalidRange,
    BonusDuplicatesWithNumbers,

    LottoInvalidSize,
    LottoDuplicate,
    LottoInvalidRange,
}

impl AppError {
    pub fn message(&self) -> &str {
        match self {
            Self::InputEmpty => "[ERROR] 입력값이 비어있습니다.",
            Self::InputNotPositive => "[ERROR] 입력값은 양수여야 합니다.",
            Self::InputNotCharacter => "[ERROR] 입력값은 char 타입이어야 합니다.",

            Self::MoneyTooSmall => "[ERROR] 구매 금액은 1000원 이상이어야 합니다.",
            Self::MoneyInvalidUnit => "[ERROR] 구매 금액은 1000원 단위여야 합니다.",

            Self::SelectionInvalidMode => "[ERROR] 로또 번호 선택 모드는 a 또는 m이어야 합니다.",

            Self::NumbersInvalidSize => "[ERROR] 당첨 번호는 6개여야 합니다.",
            Self::NumbersDuplicate => "[ERROR] 당첨 번호 중 중복된 숫자가 있습니다.",
            Self::NumbersInvalidRange => "[ERROR] 당첨 번호는 1에서 45 사이여야 합니다.",

            Self::BonusInvalidRange => "[ERROR] 보너스 번호는 1에서 45 사이여야 합니다.",
            Self::BonusDuplicatesWithNumbers => "[ERROR] 보너스 번호가 당첨 번호와 중복됩니다.",

            Self::LottoInvalidSize => "[ERROR] 로또 번호는 6개여야 합니다.",
            Self::LottoDuplicate => "[ERROR] 로또 번호 중 중복된 숫자가 있습니다.",
            Self::LottoInvalidRange => "[ERROR] 로또 번호는 1에서 45 사이여야 합니다.",
        }
    }
}
