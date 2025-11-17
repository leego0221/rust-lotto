#[derive(Debug)]
pub enum InputError {
    Empty,
    NotNumber,
    TooSmall,
    InvalidUnit,
}

impl InputError {
    pub fn message(&self) -> &str {
        match self {
            InputError::Empty => "[ERROR] 입력값이 비어있습니다.",
            InputError::NotNumber => "[ERROR] 입력값이 숫자가 아닙니다.",
            InputError::TooSmall => "[ERROR] 입력값이 TooSmall.",
            InputError::InvalidUnit => "[ERROR] 입력값이 InvalidUnit.",
        }
    }
}