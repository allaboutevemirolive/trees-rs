use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

pub type TResult<T> = Result<T, Box<dyn TError>>;

pub trait TError: Error + Send + Sync + 'static {
    fn code(&self) -> i32;
}

#[derive(Debug)]
pub struct TSimpleError {
    pub code: i32,
    pub message: String,
}

impl TSimpleError {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: Into<String>>(code: i32, message: S) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl Error for TSimpleError {}

impl Display for TSimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.code {
            1 => write!(f, "IO Error: {}", self.message),
            2 => write!(f, "Symlink Error: {}", self.message),
            _ => write!(f, "Error: {}", self.message),
        }
    }
}

impl TError for TSimpleError {
    fn code(&self) -> i32 {
        self.code
    }
}

impl From<TSimpleError> for Box<dyn TError> {
    fn from(err: TSimpleError) -> Self {
        Box::new(err)
    }
}

impl From<std::io::Error> for Box<dyn TError> {
    fn from(err: std::io::Error) -> Self {
        Box::new(TSimpleError::new(1, format!("IO Error: {}", err)))
    }
}

impl<T> From<Option<T>> for Box<dyn TError> {
    fn from(err: Option<T>) -> Self {
        match err {
            Some(_) => Box::new(TSimpleError::new(0, "Unexpected Value".to_string())),
            None => Box::new(TSimpleError::new(404, "Value Not Found".to_string())),
        }
    }
}

#[allow(dead_code)]
fn find_resource(id: u64) -> Result<(), Box<dyn TError>> {
    Err(TSimpleError::new(404, format!("Resource with id {} not found", id)).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usimple_error_message() {
        let error = TSimpleError::new(404, "Not Found");
        assert_eq!(error.to_string(), "Error: Not Found");
    }

    #[test]
    fn test_usimple_error_code() {
        let error = TSimpleError::new(404, "Not Found");
        assert_eq!(error.code(), 404);
    }

    #[test]
    fn test_find_resource_error() {
        let result = find_resource(123);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.code(), 404);
        assert_eq!(err.to_string(), "Error: Resource with id 123 not found");
    }
}
