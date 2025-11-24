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
            return Err(AppError::ManualInvalidSize(maximum))
        }

        Ok(())
    }
}