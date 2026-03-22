use elysium_rust::common::v1::ErrorCode;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Error(elysium_rust::common::v1::Error);

impl Error {
    pub fn new(code: ErrorCode, message: impl ToString) -> Self {
        Self(elysium_rust::common::v1::Error {
            code: code as i32,
            message: message.to_string(),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            ErrorCode::try_from(self.0.code)
                .map(|code| code.as_str_name())
                .unwrap_or("INVALID_ERROR_CODE"),
            self.0.message
        )
    }
}

impl std::error::Error for Error {}

impl From<elysium_rust::common::v1::Error> for Error {
    fn from(value: elysium_rust::common::v1::Error) -> Self {
        Self(value)
    }
}

impl Into<elysium_rust::common::v1::Error> for Error {
    fn into(self) -> elysium_rust::common::v1::Error {
        self.0
    }
}

impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        Self::new(ErrorCode::Internal, value)
    }
}
