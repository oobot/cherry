#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("{0}")]
    AnyHow(#[from] anyhow::Error),
    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn from<S: Into<String>>(s: S) -> Self {
        Error::Other(s.into())
    }
}


#[macro_export]
macro_rules! cherry_err {
    ($msg:literal $(,)?) => {
        crate::error::Error::from($msg)
    };
    ($err:expr $(,)?) => ({
        crate::error::Error::from($err)
    });
    ($fmt:expr, $($arg:tt)*) => {
        crate::error::Error::from(format!($fmt, $($arg)*))
    };
}
