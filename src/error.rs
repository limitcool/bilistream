pub type AppError = Box<dyn std::error::Error + Send + Sync>;
pub type AppResult<T> = Result<T, AppError>;
