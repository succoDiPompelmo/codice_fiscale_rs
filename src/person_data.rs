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

impl Gender {
    pub fn new(gender: &str) -> Gender {
        match gender {
            "F" => Gender::F,
            "M" => Gender::M,
            _ => Gender::F,
        }
    }
}
