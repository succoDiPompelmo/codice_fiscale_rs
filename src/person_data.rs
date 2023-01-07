use chrono::NaiveDate;

use crate::{
    errors::PersonDataError,
    verifier::{verify_ascii_alphanumeric, verify_birth_place_part},
};

type Result<T> = std::result::Result<T, PersonDataError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonData {
    pub name: String,
    pub surname: String,
    pub birthdate: NaiveDate,
    pub gender: Gender,
    pub place_of_birth: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    M,
    F,
}

impl PersonData {
    pub fn new(
        name: String,
        surname: String,
        birthdate: NaiveDate,
        gender: Gender,
        place_of_birth: String,
    ) -> Result<PersonData> {
        if verify_ascii_alphanumeric(&name).is_err() {
            return Err(PersonDataError::InvalidName());
        }

        if verify_ascii_alphanumeric(&surname).is_err() {
            return Err(PersonDataError::InvalidSurname());
        }

        if verify_birth_place_part(&place_of_birth).is_err() {
            return Err(PersonDataError::InvalidBirthPlace());
        }

        Ok(PersonData {
            name,
            surname,
            birthdate,
            gender,
            place_of_birth,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn person_data_new() {
        let naive_now = Utc::now().date_naive();
        let person_data = PersonData::new(
            "PIPPO".to_string(),
            "PLUTO".to_string(),
            naive_now,
            Gender::F,
            "T567".to_string(),
        );

        assert_eq!(
            person_data,
            Ok(PersonData {
                name: "PIPPO".to_string(),
                surname: "PLUTO".to_string(),
                birthdate: naive_now,
                gender: Gender::F,
                place_of_birth: "T567".to_string()
            })
        );
    }

    #[test]
    fn person_data_invalid_name() {
        let naive_now = Utc::now().date_naive();
        let person_data = PersonData::new(
            "PòPPO".to_string(),
            "PLUTO".to_string(),
            naive_now,
            Gender::F,
            "T567".to_string(),
        );

        assert_eq!(person_data, Err(PersonDataError::InvalidName()));
    }

    #[test]
    fn person_data_invalid_surname() {
        let naive_now = Utc::now().date_naive();
        let person_data = PersonData::new(
            "PIPPO".to_string(),
            "@LUTO".to_string(),
            naive_now,
            Gender::F,
            "T567".to_string(),
        );

        assert_eq!(person_data, Err(PersonDataError::InvalidSurname()));
    }

    #[test]
    fn person_data_invalid_birth_place() {
        let naive_now = Utc::now().date_naive();
        let person_data = PersonData::new(
            "PIPPO".to_string(),
            "PLUTO".to_string(),
            naive_now,
            Gender::F,
            "Ta67".to_string(),
        );

        assert_eq!(person_data, Err(PersonDataError::InvalidBirthPlace()));
    }
}
