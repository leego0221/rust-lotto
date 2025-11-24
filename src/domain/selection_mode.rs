use crate::error::AppError;

#[derive(Debug)]
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
