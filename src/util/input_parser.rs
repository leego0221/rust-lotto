use crate::error::errors::AppError;

pub fn parse_unsigned_integer(input: &str) -> Result<u32, AppError> {
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        return Err(AppError::InputEmpty)
    }

    match trimmed_input.parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(AppError::InputNotPositive),
    }
}

pub fn parse_winning_number(input: &str) -> Result<Vec<u32>, AppError> {
    input.split(",")
        .map(parse_unsigned_integer)
        .collect()
}