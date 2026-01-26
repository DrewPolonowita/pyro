use owo_colors::OwoColorize;

pub type Result<T> = core::result::Result<T, AppError>;

// new error system

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    HintError(HintError),
    Error(Error)
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<HintError> for AppError {
    fn from(err: HintError) -> Self {
        AppError::HintError(err)
    }
}

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError::Error(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(io_err) => write!(f, "{}", display_io_error(io_err)),
            AppError::HintError(err) => write!(f, "{}", err),
            AppError::Error(err) => write!(f, "{}", err)
        }
    }
}

#[derive(Debug)]
pub enum ErrorType {
    AlreadyExists,
    NotFound,
    NotEmpty,
    InvalidFlag,
    InvalidToken,
    EmptyString
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorType::AlreadyExists => write!(f, "already exists"),
            ErrorType::NotFound => write!(f, "not found"),
            ErrorType::NotEmpty => write!(f, "not empty"),
            ErrorType::InvalidFlag => write!(f, "invalid flag"),
            ErrorType::InvalidToken => write!(f, "invalid token"),
            ErrorType::EmptyString => write!(f, "empty string"),
        }
    }
}

#[derive(Debug)]
pub struct HintError {
    pub error_type: ErrorType,
    pub error_message: &'static str,
    pub hint_message: &'static str
}

impl std::fmt::Display for HintError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_message = format!("\nerror {}: {}\n{} {}\n", self.error_type.to_string().red(), self.error_message, "Hint:".yellow(), self.hint_message);
        write!(f, "{}", error_message)
    }
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub error_message: &'static str,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error_message = format!("\nerror {}: {}\n", self.error_type.to_string().red(), self.error_message);
        write!(f, "{}", error_message)
    }
}

pub fn display_io_error(e: &std::io::Error) -> String {
    format!("\nerror {}: {}\n", e.kind().red(), e)
}