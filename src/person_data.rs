use chrono::NaiveDate;

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
    ) -> PersonData {
        PersonData {
            name,
            surname,
            birthdate,
            gender,
            place_of_birth,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn valid_codice_fiscale() {
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
            PersonData {
                name: "PIPPO".to_string(),
                surname: "PLUTO".to_string(),
                birthdate: naive_now,
                gender: Gender::F,
                place_of_birth: "T567".to_string()
            }
        );
    }
}
