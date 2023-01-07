use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::{
    common::month_codes, control_code::ControlCode, errors::VerifierError, omocodes::Omocodes,
};

type Result<T> = std::result::Result<T, VerifierError>;

pub struct Verifier {}

#[derive(Error, Debug, PartialEq, Eq)]
pub struct VerifierOutcome {
    codice_fiscale: String,
}

impl fmt::Display for VerifierOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl Verifier {
    pub fn verify(codice_fiscale: &str) -> Result<VerifierOutcome> {
        if codice_fiscale.len() != 16 {
            return Err(VerifierError::InvalidLength(codice_fiscale.len()));
        }

        verify_characters(codice_fiscale)?;

        let purified_codice_fiscale = Omocodes::replace_omocodes_characters(codice_fiscale);

        verify_surname_part(&purified_codice_fiscale[0..3])?;
        verify_name_part(&purified_codice_fiscale[3..6])?;
        verify_birth_year_part(&purified_codice_fiscale[6..8])?;
        verify_birth_month_part(&purified_codice_fiscale[8..9])?;
        verify_birth_day_and_gender_part(&purified_codice_fiscale[9..11])?;
        verify_birth_place_part(&purified_codice_fiscale[11..15])?;

        verify_control_code(codice_fiscale)?;

        Ok(VerifierOutcome {
            codice_fiscale: codice_fiscale.to_string(),
        })
    }
}

impl VerifierOutcome {
    pub fn get(&self) -> String {
        self.codice_fiscale.to_string()
    }
}

fn verify_characters(codice_fiscale: &str) -> Result<()> {
    if let Some(invalid_character_position) = codice_fiscale
        .as_bytes()
        .iter()
        .position(|c| !c.is_ascii_alphanumeric())
    {
        return Err(VerifierError::NonAlphanumericCharacter(
            invalid_character_position,
        ));
    }

    Ok(())
}

fn verify_surname_part(surname_part: &str) -> Result<()> {
    match surname_part.as_bytes() {
        &[a, b, c]
            if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() && c.is_ascii_alphabetic() =>
        {
            Ok(())
        }
        _ => Err(VerifierError::InvalidSurname(surname_part.to_string())),
    }
}

fn verify_name_part(name_part: &str) -> Result<()> {
    match name_part.as_bytes() {
        &[a, b, c]
            if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() && c.is_ascii_alphabetic() =>
        {
            Ok(())
        }
        _ => Err(VerifierError::InvalidName(name_part.to_string())),
    }
}

fn verify_birth_year_part(birth_year_part: &str) -> Result<()> {
    let _result: u32 = FromStr::from_str(birth_year_part)
        .map_err(|_| VerifierError::InvalidBirthYear(birth_year_part.to_string()))?;

    Ok(())
}

fn verify_birth_month_part(birth_month_part: &str) -> Result<()> {
    match birth_month_part
        .chars()
        .all(|char| month_codes().contains(&char))
    {
        true => Ok(()),
        false => Err(VerifierError::InvalidBirthMonth(
            birth_month_part.to_string(),
        )),
    }
}

fn verify_birth_day_and_gender_part(birth_day_and_gender_part: &str) -> Result<()> {
    let birth_day: u32 = FromStr::from_str(birth_day_and_gender_part).map_err(|_| {
        VerifierError::InvalidBirthDayAndGender(birth_day_and_gender_part.to_string())
    })?;
    if (1..=31).contains(&birth_day) || (41..=71).contains(&birth_day) {
        return Ok(());
    }

    Err(VerifierError::InvalidBirthDayAndGenderRange(birth_day))
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
        _ => Err(VerifierError::InvalidBirthPlace(
            birth_place_part.to_string(),
        )),
    }
}

fn verify_control_code(codice_fiscale: &str) -> Result<()> {
    let expected_control_code = ControlCode::compute(codice_fiscale);
    let control_code = codice_fiscale.chars().last();

    match (expected_control_code, control_code) {
        (expected, Some(value)) if expected == value => Ok(()),
        (expected, Some(value)) => Err(VerifierError::InvalidControlCharacter(value, expected)),
        (expected, None) => Err(VerifierError::InvalidControlCharacter(' ', expected)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_codice_fiscale() {
        assert!(Verifier::verify("cTMTBT74E05B506W").is_ok())
    }

    #[test]
    fn valid_codice_fiscale_omocodo() {
        assert_eq!(
            Verifier::verify("BRNPRZ72D52F83VC"),
            Ok(VerifierOutcome {
                codice_fiscale: "BRNPRZ72D52F83VC".to_string()
            })
        )
    }

    #[test]
    fn invalid_codice_fiscale_not_alphanumeric_character() {
        assert_eq!(
            Verifier::verify("CTmTBT7?E05B506Y"),
            Err(VerifierError::NonAlphanumericCharacter(7))
        )
    }

    #[test]
    fn invalid_codice_fiscale_control_code() {
        assert_eq!(
            Verifier::verify("CTmTBT74E05B506Y"),
            Err(VerifierError::InvalidControlCharacter('Y', 'W'))
        )
    }

    #[test]
    fn invalid_codice_fiscale_length() {
        assert_eq!(
            Verifier::verify("CTMTB"),
            Err(VerifierError::InvalidLength(5))
        )
    }

    #[test]
    fn invalid_codice_fiscale_surname_part() {
        assert_eq!(
            Verifier::verify("CT0TBT74E05B506W"),
            Err(VerifierError::InvalidSurname("CT0".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_name_part() {
        assert_eq!(
            Verifier::verify("CTMTB174E05B506W"),
            Err(VerifierError::InvalidName("TB1".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_year_part() {
        assert_eq!(
            Verifier::verify("CTMTBTy4E05B506W"),
            Err(VerifierError::InvalidBirthYear("y4".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_month_part() {
        assert_eq!(
            Verifier::verify("CTMTBT74X05B506W"),
            Err(VerifierError::InvalidBirthMonth("X".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_day_and_gender_part() {
        assert_eq!(
            Verifier::verify("CTMTBT74EF5B506W"),
            Err(VerifierError::InvalidBirthDayAndGender("F5".to_string()))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_day_and_gender_range_part() {
        assert_eq!(
            Verifier::verify("CTMTBT74E32B506W"),
            Err(VerifierError::InvalidBirthDayAndGenderRange(32))
        )
    }

    #[test]
    fn invalid_codice_fiscale_birth_place_part() {
        assert_eq!(
            Verifier::verify("CTMTBT74E31B5F6W"),
            Err(VerifierError::InvalidBirthPlace("B5F6".to_string()))
        )
    }
}
