#[derive(Debug, thiserror::Error)]
pub enum CherryError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("{0}")]
    AnyHow(#[from] anyhow::Error),
    #[error("{0}")]
    Other(String),
}

impl CherryError {
    pub(crate) fn from<S: Into<String>>(s: S) -> Self {
        CherryError::Other(s.into())
    }
}


#[macro_export]
macro_rules! cherry_err {
    ($msg:literal $(,)?) => {
        crate::error::CherryError::from($msg)
    };
    ($err:expr $(,)?) => ({
        crate::error::CherryError::from($err)
    });
    ($fmt:expr, $($arg:tt)*) => {
        crate::error::CherryError::from(format!($fmt, $($arg)*))
    };
}
