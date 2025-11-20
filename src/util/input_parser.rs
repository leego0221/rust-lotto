use crate::error::AppError;

pub struct InputParser;

impl InputParser {
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
    fn valid_winning_number_input() {
        // given
        let input = "1, 2, 3, 4, 5, 6";

        // when
        let result = InputParser::parse_winning_number(input);

        // then
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn winning_number_input_empty() {
        // given
        let input = "1,2,3,4,,6";

        // when
        let result = InputParser::parse_winning_number(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputEmpty);
    }

    #[test]
    fn winning_number_input_not_positive() {
        // given
        let input = "-1,2,3,4,5,6";

        // when
        let result = InputParser::parse_winning_number(input);

        // then
        assert_eq!(result.unwrap_err(), AppError::InputNotPositive);
    }
}