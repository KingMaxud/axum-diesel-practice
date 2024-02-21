use deadpool_diesel::{InteractError, PoolError};
use std::fmt;

#[derive(Debug)]
pub enum InfraError {
    InternalServerError,
    NotFound,
}

// Utility function to adapt errors of generic type T into InfraError
pub fn adapt_infra_error<T: Error>(error: T) -> InfraError {
    error.as_infra_error()
}

// Implement the Display trait to customize how InfraError is displayed
impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfraError::NotFound => write!(f, "Not found"),
            InfraError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

// Define a custom Error trait for types that can be converted to InfraError
pub trait Error {
    fn as_infra_error(&self) -> InfraError;
}

// Implement the Error trait for diesel::result::Error
impl Error for diesel::result::Error {
    fn as_infra_error(&self) -> InfraError {
        match self {
            diesel::result::Error::NotFound => InfraError::NotFound,
            _ => InfraError::InternalServerError,
        }
    }
}

// Implement the Error trait for deadpool_diesel::PoolError
impl Error for PoolError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError
    }
}

// Implement the Error trait for InteractError
impl Error for InteractError {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError // Map all InteractError instances to InfraError::InternalServerError
    }
}

impl Error for dotenvy::Error {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError // Map all InteractError instances to InfraError::InternalServerError
    }
}

impl Error for &str {
    fn as_infra_error(&self) -> InfraError {
        InfraError::InternalServerError // Map all InteractError instances to InfraError::InternalServerError
    }
}
