use postgres::error::Error as PostgresError;
use std::io::Error as IoError;

pub type Result<T> = std::result::Result<T, ErpError>;

#[derive(Debug, Fail)]
pub enum ErpError {
    #[fail(display = "{}", _0)]
    Io(#[cause] IoError),
    #[fail(display = "{}", _0)]
    Postgres(#[cause] PostgresError),
    #[fail(display = "Received request was badly formatted.")]
    BadRequest,
    #[fail(display = "Resource not found.")]
    NotFound,
}

impl From<PostgresError> for ErpError {
    fn from(err: PostgresError) -> Self {
        Self::Postgres(err)
    }
}
