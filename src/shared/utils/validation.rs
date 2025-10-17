use crate::shared::errors::DomainError;

pub fn ensure_not_empty(value: &str, field: &str) -> Result<(), DomainError> {
    if value.trim().is_empty() { Err(DomainError::Validation(format!("{field} can't be empty"))) } else { Ok(()) }
}

