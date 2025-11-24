use crate::error::AppError;

#[derive(Debug)]
pub struct ManualCount {
    count: u32
}

impl ManualCount {
    pub fn new(count: u32, maximum: u32) -> Result<ManualCount, AppError> {
        Self::validate(count, maximum)?;
        Ok(ManualCount { count })
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    fn validate(count: u32, maximum: u32) -> Result<(), AppError> {
        if count == 0 || count > maximum {
            return Err(AppError::ManualInvalidRange(maximum))
        }

        Ok(())
    }
}

#[cfg(test)]
mod manual_count_tests {
    use super::*;

    #[test]
    fn valid_manual_count() {
        // given
        let count = 5;
        let maximum = 7;

        // when
        let result = ManualCount::new(count, maximum);
        
        // then
        assert_eq!(result.unwrap().count(), 5);
    }

    #[test]
    fn manual_count_range() {
        // given
        let count = 5;
        let maximum = 3;

        // when
        let result = ManualCount::new(count, maximum);

        // then
        assert_eq!(result.unwrap_err(), AppError::ManualInvalidRange(3));
    }
}
