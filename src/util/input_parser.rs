use crate::error::input_error::InputError;

pub fn parse_unsigned_integer(input: &str) -> Result<u32, InputError> {
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        return Err(InputError::Empty)
    }

    match trimmed_input.parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(InputError::NotNumber),
    }
}

pub fn parse_winning_number(input: &str) -> Result<Vec<u32>, InputError> {
    input.split(",")
        .map(parse_unsigned_integer)
        .collect()
}