use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectorError {
    #[error("Command execution failed: {0}")]
    CommandFailed(String),
    
    #[error("Failed to parse log entry: {0}")]
    ParseError(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl CollectorError {
    pub fn is_permission_error(&self) -> bool {
        matches!(self, CollectorError::PermissionDenied(_))
    }

    pub fn is_rate_limit_error(&self) -> bool {
        matches!(self, CollectorError::RateLimitExceeded(_))
    }
}