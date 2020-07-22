use std::fmt;

#[derive(Debug, Clone)]
pub struct DeviceNotFoundError;

impl std::error::Error for DeviceNotFoundError {}
impl fmt::Display for DeviceNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input device not found")
    }
}
