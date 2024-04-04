use std::error::Error;
use std::fmt::{Display, Formatter};

pub type UResult<T> = Result<T, Box<dyn UError>>;

pub trait UError: Error + Send + Sync + 'static {
    fn code(&self) -> i32;
}

#[derive(Debug)]
pub struct USimpleError {
    pub code: i32,
    pub message: String,
}

impl USimpleError {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: Into<String>>(code: i32, message: S) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl Error for USimpleError {}

impl Display for USimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.message.fmt(f)
    }
}

impl UError for USimpleError {
    fn code(&self) -> i32 {
        self.code
    }
}

impl From<USimpleError> for Box<dyn UError> {
    fn from(err: USimpleError) -> Self {
        Box::new(err)
    }
}

impl From<std::io::Error> for Box<dyn UError> {
    fn from(err: std::io::Error) -> Self {
        Box::new(USimpleError::new(1, format!("IO Error: {}", err)))
    }
}

// impl<T> From<Option<T>> for Box<dyn UError>
// where
//     T: Into<Box<dyn UError>>,
// {
//     fn from(option: Option<T>) -> Self {
//         match option {
//             Some(inner) => inner.into(),
//             None => Box::new(USimpleError::new(
//                 1,
//                 "Failed to get relative path".to_string(),
//             )),
//         }
//     }
// }

#[allow(dead_code)]
fn find_resource(id: u64) -> Result<(), Box<dyn UError>> {
    Err(USimpleError::new(404, format!("Resource with id {} not found", id)).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usimple_error_message() {
        let error = USimpleError::new(404, "Not Found");
        assert_eq!(error.to_string(), "Not Found");
    }

    #[test]
    fn test_usimple_error_code() {
        let error = USimpleError::new(404, "Not Found");
        assert_eq!(error.code(), 404);
    }

    #[test]
    fn test_find_resource_error() {
        let result = find_resource(123);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.code(), 404);
        assert_eq!(err.to_string(), "Resource with id 123 not found");
    }
}
