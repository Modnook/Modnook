#[derive(Debug, thiserror::Error)]
pub enum ModnookError {
    #[error("A database error occurred")]
    Toasty(#[from] toasty::Error),
}

pub type ModnookResult<T> = std::result::Result<T, ModnookError>;
