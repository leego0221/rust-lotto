use crate::error::AppError;

pub struct InputParser;

impl InputParser {
    pub fn parse_unsigned_integer(input: &str) -> Result<u32, AppError> {
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            return Err(AppError::InputEmpty)
        }

        match trimmed_input.parse::<u32>() {
            Ok(v) => Ok(v),
            Err(_) => Err(AppError::InputNotPositive),
        }
    }

    pub fn parse_character(input: &str) -> Result<char, AppError> {
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            return Err(AppError::InputEmpty)
        }

        match trimmed_input.parse::<char>() {
            Ok(v) => Ok(v),
            Err(_) => Err(AppError::InputNotCharacter),
        }
    }

    pub fn parse_numbers(input: &str) -> Result<Vec<u32>, AppError> {
        input.split(",")
            .map(Self::parse_unsigned_integer)
            .collect()
    }
}

#[cfg(test)]
mod input_parser_tests {
    use super::*;

    #[test]
    fn valid_unsigned_integer_input() {
        // given
        let input = "5";

        // when
        let result = InputParser::parse_unsigned_integer(input);

        // then
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn unsigned_integer_input_empty() {
        // given
        let input = "";

        // when
        let result = InputParser::parse_unsigned_integer(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputEmpty);
    }

    #[test]
    fn unsigned_integer_input_not_positive() {
        // given
        let input = "-100";

        // when
        let result = InputParser::parse_unsigned_integer(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputNotPositive);
    }

    #[test]
    fn valid_character_input() {
        // given
        let input = "w";

        // when
        let result = InputParser::parse_character(input);

        // then
        assert_eq!(result.unwrap(), 'w');
    }

    #[test]
    fn character_input_empty() {
        // given
        let input = " ";

        // when
        let result = InputParser::parse_character(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputEmpty);
    }

    #[test]
    fn character_input_not_char() {
        // given
        let input = "str";

        // when
        let result = InputParser::parse_character(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputNotCharacter);
    }

    #[test]
    fn valid_numbers_input() {
        // given
        let input = "1, 2, 3, 4, 5, 6";

        // when
        let result = InputParser::parse_numbers(input);

        // then
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn numbers_input_empty() {
        // given
        let input = "1,2,3,4,,6";

        // when
        let result = InputParser::parse_numbers(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputEmpty);
    }

    #[test]
    fn numbers_input_not_positive() {
        // given
        let input = "-1,2,3,4,5,6";

        // when
        let result = InputParser::parse_numbers(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputNotPositive);
    }
}
