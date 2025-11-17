#[derive(Debug)]
pub enum InputError {
    Empty,
    NotNumber,
}

impl InputError {
    pub fn message(&self) -> &str {
        match self {
            InputError::Empty => "[ERROR] 입력값이 비어있습니다.",
            InputError::NotNumber => "[ERROR] 입력값은 숫자여야 합니다.",
        }
    }
}

#[derive(Debug)]
pub enum DomainError {
    MoneyTooSmall,
    MoneyInvalidUnit,
    NumbersInvalidSize,
    NumbersDuplicate,
    NumbersInvalidRange,
    BonusInvalidRange,
    BonusDuplicatesWithNumbers,
}

impl DomainError {
    pub fn message(&self) -> &str {
        match self {
            DomainError::MoneyTooSmall => "[ERROR] 구매 금액은 1000원 이상이어야 합니다.",
            DomainError::MoneyInvalidUnit => "[ERROR] 구매 금액은 1000원 단위여야 합니다.",
            DomainError::NumbersInvalidSize => "[ERROR] 당첨 번호는 6개여야 합니다.",
            DomainError::NumbersDuplicate => "[ERROR] 당첨 번호 중 중복된 숫자가 있습니다.",
            DomainError::NumbersInvalidRange => "[ERROR] 당첨 번호는 1에서 45 사이여야 합니다.",
            DomainError::BonusInvalidRange => "[ERROR] 보너스 번호는 1에서 45 사이여야 합니다.",
            DomainError::BonusDuplicatesWithNumbers => "[ERROR] 보너스 번호가 당첨 번호와 중복됩니다.",
        }
    }
}