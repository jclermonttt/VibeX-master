use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Custom(String),

    // -- Module Extern
    #[from]
    DbErr(sea_orm::error::DbErr),
}

impl Error {
    pub fn new<S: Into<String>>(msg: S) -> Self {
        Error::Custom(msg.into())
    }
}

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
