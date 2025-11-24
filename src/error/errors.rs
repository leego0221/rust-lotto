#[derive(Debug, PartialEq)]
pub enum AppError {
    InputEmpty,
    InputNotPositive,
    InputNotCharacter,

    MoneyTooSmall,
    MoneyInvalidUnit,

    SelectionInvalidMode,

    ManualInvalidSize(u32),

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
    pub fn message(&self) -> String {
        match self {
            Self::InputEmpty => String::from("[ERROR] 입력값이 비어있습니다."),
            Self::InputNotPositive => String::from("[ERROR] 입력값은 양수여야 합니다."),
            Self::InputNotCharacter => String::from("[ERROR] 입력값은 char 타입이어야 합니다."),

            Self::MoneyTooSmall => String::from("[ERROR] 구매 금액은 1000원 이상이어야 합니다."),
            Self::MoneyInvalidUnit => String::from("[ERROR] 구매 금액은 1000원 단위여야 합니다."),

            Self::SelectionInvalidMode => String::from("[ERROR] 로또 번호 선택 모드는 a 또는 m이어야 합니다."),

            Self::ManualInvalidSize(maximum) => format!("[ERROR] 수동 선택 개수는 1에서 {} 사이여야 합니다.", maximum),

            Self::NumbersInvalidSize => String::from("[ERROR] 당첨 번호는 6개여야 합니다."),
            Self::NumbersDuplicate => String::from("[ERROR] 당첨 번호 중 중복된 숫자가 있습니다."),
            Self::NumbersInvalidRange => String::from("[ERROR] 당첨 번호는 1에서 45 사이여야 합니다."),

            Self::BonusInvalidRange => String::from("[ERROR] 보너스 번호는 1에서 45 사이여야 합니다."),
            Self::BonusDuplicatesWithNumbers => String::from("[ERROR] 보너스 번호가 당첨 번호와 중복됩니다."),

            Self::LottoInvalidSize => String::from("[ERROR] 로또 번호는 6개여야 합니다."),
            Self::LottoDuplicate => String::from("[ERROR] 로또 번호 중 중복된 숫자가 있습니다."),
            Self::LottoInvalidRange => String::from("[ERROR] 로또 번호는 1에서 45 사이여야 합니다."),
        }
    }
}
