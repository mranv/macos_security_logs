use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectorError {
    #[error("Command execution failed: {0}")]
    CommandFailed(String),
    
    #[error("Failed to parse log entry: {0}")]
    ParseError(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}
