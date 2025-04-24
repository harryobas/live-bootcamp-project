pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    InCorrectCredentials,
    MissingToken,
    InvalidToken,
}