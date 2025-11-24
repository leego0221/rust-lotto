use crate::error::AppError;

#[derive(Debug, PartialEq)]
pub enum SelectionMode {
    Auto,
    Manual,
}

impl SelectionMode {
    pub fn from(mode: char) -> Result<SelectionMode, AppError> {
        match mode {
            'a' => Ok(SelectionMode::Auto),
            'm' => Ok(SelectionMode::Manual),
            _ => Err(AppError::SelectionInvalidMode),
        }
    }
}

#[cfg(test)]
mod selection_mode_tests {
    use super::*;

    #[test]
    fn from_test_success() {
        // given
        let mode_auto = 'a';
        let mode_manual = 'm';

        // when
        let result1 = SelectionMode::from(mode_auto);
        let result2 = SelectionMode::from(mode_manual);

        // then
        assert_eq!(result1.unwrap(), SelectionMode::Auto);
        assert_eq!(result2.unwrap(), SelectionMode::Manual);
    }

    #[test]
    fn from_test_fail() {
        // given
        let mode_invalid = 'i';
        
        // when
        let result = SelectionMode::from(mode_invalid);

        // then
        assert_eq!(result.unwrap_err(), AppError::SelectionInvalidMode);
    }
}
