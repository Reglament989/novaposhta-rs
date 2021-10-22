use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct NovaRequestError {
    details: String,
}

impl NovaRequestError {
    pub fn new(msg: String) -> NovaRequestError {
        NovaRequestError {
            details: msg.to_string(),
        }
    }
}

impl Display for NovaRequestError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NovaRequestError {
    fn description(&self) -> &str {
        &self.details
    }
}
