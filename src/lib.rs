use std::fmt;
use std::str::FromStr;

use rand::distributions::{Distribution, Uniform};
use thiserror::Error;

const ALPHABET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const MONTH_CODES: &[char] = &['A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'P', 'R', 'S', 'T'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonData {
    pub name: String,
    pub surname: String,
    pub birthdate: String,
    pub gender: String,
    pub place_of_birth: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodiceFiscale {
    codice_fiscale: String,
}

impl fmt::Display for CodiceFiscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

type Result<T> = std::result::Result<T, CodiceFiscaleError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CodiceFiscaleError {
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

impl CodiceFiscale {
    pub fn verify(codice_fiscale: &str) -> Result<CodiceFiscale> {
        if codice_fiscale.len() != 16 {
            return Err(CodiceFiscaleError::InvalidLength(codice_fiscale.len()));
        }

        if let Some(invalid_character_position) = codice_fiscale
            .as_bytes()
            .iter()
            .position(|c| !c.is_ascii_alphanumeric())
        {
            return Err(CodiceFiscaleError::NonAlphanumericCharacter(
                invalid_character_position,
            ));
        }

        verify_surname_part(&codice_fiscale[0..3])?;
        verify_name_part(&codice_fiscale[3..6])?;
        verify_birth_year_part(&codice_fiscale[6..8])?;
        verify_birth_month_part(&codice_fiscale[8..9])?;
        verify_birth_day_and_gender_part(&codice_fiscale[9..11])?;
        verify_birth_place_part(&codice_fiscale[11..15])?;

        verify_control_code(codice_fiscale)?;

        Ok(CodiceFiscale {
            codice_fiscale: codice_fiscale.to_string(),
        })
    }

    pub fn generate_random() -> CodiceFiscale {
        let mut codice_fiscale = vec![];

        let mut rng = rand::thread_rng();
        let alphabet_index = Uniform::from(1..26);
        let digit = Uniform::from(0..10);
        let month_codes_index = Uniform::from(0..12);
        let day_first_digit = Uniform::from(1..2);
        let day_second_digit = Uniform::from(0..10);
        let gender = Uniform::from(0..2);

        for _i in 0..6 {
            codice_fiscale.push(ALPHABET[alphabet_index.sample(&mut rng)])
        }

        for _i in 0..2 {
            codice_fiscale.push(char::from_digit(digit.sample(&mut rng), 10).unwrap());
        }

        codice_fiscale.push(MONTH_CODES[month_codes_index.sample(&mut rng)]);
        codice_fiscale.push(
            char::from_digit(
                day_first_digit.sample(&mut rng) + 4 * gender.sample(&mut rng),
                10,
            )
            .unwrap(),
        );
        codice_fiscale.push(char::from_digit(day_second_digit.sample(&mut rng), 10).unwrap());
        codice_fiscale.push(ALPHABET[alphabet_index.sample(&mut rng)]);
        for _i in 0..3 {
            codice_fiscale.push(char::from_digit(digit.sample(&mut rng), 10).unwrap());
        }

        let value: String = codice_fiscale.iter().collect();
        let control_code = compute_control_code(&value);

        codice_fiscale.push(control_code);
        let codice_fiscale: String = codice_fiscale.iter().collect();
        CodiceFiscale { codice_fiscale }
    }

    pub fn get(&self) -> String {
        self.codice_fiscale.to_string()
    }
}

fn verify_surname_part(surname_part: &str) -> Result<()> {
    match surname_part.as_bytes() {
        &[a, b, c]
            if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() && c.is_ascii_alphabetic() =>
        {
            Ok(())
        }
        _ => Err(CodiceFiscaleError::InvalidSurname(surname_part.to_string())),
    }
}

fn verify_name_part(name_part: &str) -> Result<()> {
    match name_part.as_bytes() {
        &[a, b, c]
            if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() && c.is_ascii_alphabetic() =>
        {
            Ok(())
        }
        _ => Err(CodiceFiscaleError::InvalidName(name_part.to_string())),
    }
}

fn verify_birth_year_part(birth_year_part: &str) -> Result<()> {
    let _result: u32 = FromStr::from_str(birth_year_part)
        .map_err(|_| CodiceFiscaleError::InvalidBirthYear(birth_year_part.to_string()))?;

    Ok(())
}

fn verify_birth_month_part(birth_month_part: &str) -> Result<()> {
    match birth_month_part
        .chars()
        .all(|char| MONTH_CODES.contains(&char))
    {
        true => Ok(()),
        false => Err(CodiceFiscaleError::InvalidBirthMonth(
            birth_month_part.to_string(),
        )),
    }
}

fn verify_birth_day_and_gender_part(birth_day_and_gender_part: &str) -> Result<()> {
    let birth_day: u32 = FromStr::from_str(birth_day_and_gender_part).map_err(|_| {
        CodiceFiscaleError::InvalidBirthDayAndGender(birth_day_and_gender_part.to_string())
    })?;
    if (1..=31).contains(&birth_day) || (41..=71).contains(&birth_day) {
        return Ok(());
    }

    Err(CodiceFiscaleError::InvalidBirthDayAndGenderRange(birth_day))
}

fn verify_birth_place_part(birth_place_part: &str) -> Result<()> {
    match birth_place_part.as_bytes() {
        &[a, b, c, d]
            if a.is_ascii_alphabetic()
                && b.is_ascii_digit()
                && c.is_ascii_digit()
                && d.is_ascii_digit() =>
        {
            Ok(())
        }
        _ => Err(CodiceFiscaleError::InvalidBirthPlace(
            birth_place_part.to_string(),
        )),
    }
}

fn verify_control_code(codice_fiscale: &str) -> Result<()> {
    let expected_control_code = compute_control_code(codice_fiscale);
    let control_code = codice_fiscale.chars().last();

    match (expected_control_code, control_code) {
        (expected, Some(value)) if expected == value => Ok(()),
        (expected, Some(value)) => {
            Err(CodiceFiscaleError::InvalidControlCharacter(value, expected))
        }
        (expected, None) => Err(CodiceFiscaleError::InvalidControlCharacter(' ', expected)),
    }
}

fn compute_control_code(codice_fiscale: &str) -> char {
    let partial_code: Vec<char> = codice_fiscale.to_uppercase().chars().collect();
    let mut control_code = 0;

    for (n, character) in partial_code.iter().enumerate().take(15) {
        if let Some(val) = get_conversion_table_value(character, (n + 1) % 2 == 0) {
            control_code += val;
        }
    }

    let index_alphabet: usize = (&control_code % 26).try_into().unwrap();
    ALPHABET[index_alphabet]
}

fn get_conversion_table_value(character: &char, even: bool) -> Option<i32> {
    let ternary = |even_value: i32, odd_value: i32| {
        if even {
            Some(even_value)
        } else {
            Some(odd_value)
        }
    };

    match character.to_owned() {
        'A' | '0' => ternary(0, 1),
        'B' | '1' => ternary(1, 0),
        'C' | '2' => ternary(2, 5),
        'D' | '3' => ternary(3, 7),
        'E' | '4' => ternary(4, 9),
        'F' | '5' => ternary(5, 13),
        'G' | '6' => ternary(6, 15),
        'H' | '7' => ternary(7, 17),
        'I' | '8' => ternary(8, 19),
        'J' | '9' => ternary(9, 21),
        'K' => ternary(10, 2),
        'L' => ternary(11, 4),
        'M' => ternary(12, 18),
        'N' => ternary(13, 20),
        'O' => ternary(14, 11),
        'P' => ternary(15, 3),
        'Q' => ternary(16, 6),
        'R' => ternary(17, 8),
        'S' => ternary(18, 12),
        'T' => ternary(19, 14),
        'U' => ternary(20, 16),
        'V' => ternary(21, 10),
        'W' => ternary(22, 22),
        'X' => ternary(23, 25),
        'Y' => ternary(24, 24),
        'Z' => ternary(25, 23),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_codice_fiscale() {
        assert!(CodiceFiscale::verify("cTMTBT74E05B506W").is_ok())
    }

    #[test]
    fn invalid_codice_fiscale_not_alphanumeric_character() {
        assert_eq!(
            CodiceFiscale::verify("CTmTBT7?E05B506Y"),
            Err(CodiceFiscaleError::NonAlphanumericCharacter(7))
        )
    }

    #[test]
    fn invalid_codice_fiscale_control_code() {
        assert_eq!(
            CodiceFiscale::verify("CTmTBT74E05B506Y"),
            Err(CodiceFiscaleError::InvalidControlCharacter('Y', 'W'))
        )
    }

    #[test]
    fn invalid_codice_fiscale_length() {
        assert_eq!(
            CodiceFiscale::verify("CTMTB"),
            Err(CodiceFiscaleError::InvalidLength(5))
        )
    }

    #[test]
    fn invalid_codice_fiscale_surname_part() {
        assert_eq!(
            CodiceFiscale::verify("CT0TBT74E05B506W"),
            Err(CodiceFiscaleError::InvalidSurname("CT0".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_name_part() {
        assert_eq!(
            CodiceFiscale::verify("CTMTB174E05B506W"),
            Err(CodiceFiscaleError::InvalidName("TB1".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_year_part() {
        assert_eq!(
            CodiceFiscale::verify("CTMTBTy4E05B506W"),
            Err(CodiceFiscaleError::InvalidBirthYear("y4".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_month_part() {
        assert_eq!(
            CodiceFiscale::verify("CTMTBT74X05B506W"),
            Err(CodiceFiscaleError::InvalidBirthMonth("X".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_day_and_gender_part() {
        assert_eq!(
            CodiceFiscale::verify("CTMTBT74EF5B506W"),
            Err(CodiceFiscaleError::InvalidBirthDayAndGender(
                "F5".to_string()
            ))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_day_and_gender_range_part() {
        assert_eq!(
            CodiceFiscale::verify("CTMTBT74E32B506W"),
            Err(CodiceFiscaleError::InvalidBirthDayAndGenderRange(32))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_place_part() {
        assert_eq!(
            CodiceFiscale::verify("CTMTBT74E31B5F6W"),
            Err(CodiceFiscaleError::InvalidBirthPlace("B5F6".to_string()))
        )
    }

    #[test]
    fn generate_valid_random_codice_fiscale() {
        let codice_fiscale = CodiceFiscale::generate_random();
        assert!(CodiceFiscale::verify(&codice_fiscale.get()).is_ok())
    }
}
