use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum VerifierError {
    #[error("The fiscal code length should be 16 instead is `{0}`")]
    InvalidLength(usize),
    #[error("The fiscal code surname part should be 3 letters instead is `{0}`")]
    InvalidSurname(String),
    #[error("The fiscal code name part should be 3 letters instead is `{0}`")]
    InvalidName(String),
    #[error("The fiscal code birth year part should be a 2 digits number instead is `{0}`")]
    InvalidBirthYear(String),
    #[error("The fiscal code birth month part should be 1 letter instead is `{0}`")]
    InvalidBirthMonth(String),
    #[error(
        "The fiscal code birth day and gender part should be a 2 digits number instead is `{0}`"
    )]
    InvalidBirthDayAndGender(String),
    #[error("The fiscal code birth day and gender part should be a 2 digits number between 1-31 and 41-71 instead is `{0}`")]
    InvalidBirthDayAndGenderRange(u32),
    #[error("The fiscal code birth place part should be a valid belfiore code instead is `{0}`")]
    InvalidBirthPlace(String),
    #[error("The fiscal code control character is invalid, found `{0}` expected `{1}`")]
    InvalidControlCharacter(char, char),
    #[error("The fiscal code shoud not contains any non alphanumeric character, invalid character at position `{0}`")]
    NonAlphanumericCharacter(usize),
}
