use std::vec;

use chrono::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::{
    common::{alphabet, month_codes, vowels},
    control_code::ControlCode,
    omocodes::Omocodes,
    person_data::{Gender, PersonData},
};

pub struct GeneratorOutcome {
    codice_fiscale: String,
    omocodes: Vec<String>,
}

pub struct Generator {}

impl Generator {
    pub fn generate(person_data: &PersonData) -> GeneratorOutcome {
        let codice_fiscale = generate_codice_fiscale(person_data);
        let omocodes = generate_omocodes(&codice_fiscale);
        GeneratorOutcome {
            codice_fiscale,
            omocodes,
        }
    }

    pub fn generate_random() -> GeneratorOutcome {
        let mut codice_fiscale = vec![];

        let mut rng = rand::thread_rng();
        let alphabet_index = Uniform::from(1..26);
        let digit = Uniform::from(0..10);
        let month_codes_index = Uniform::from(0..12);
        let day_first_digit = Uniform::from(1..2);
        let day_second_digit = Uniform::from(0..10);
        let gender = Uniform::from(0..2);

        for _i in 0..6 {
            codice_fiscale.push(alphabet()[alphabet_index.sample(&mut rng)])
        }

        for _i in 0..2 {
            codice_fiscale.push(char::from_digit(digit.sample(&mut rng), 10).unwrap());
        }

        codice_fiscale.push(month_codes()[month_codes_index.sample(&mut rng)]);
        codice_fiscale.push(
            char::from_digit(
                day_first_digit.sample(&mut rng) + 4 * gender.sample(&mut rng),
                10,
            )
            .unwrap(),
        );
        codice_fiscale.push(char::from_digit(day_second_digit.sample(&mut rng), 10).unwrap());
        codice_fiscale.push(alphabet()[alphabet_index.sample(&mut rng)]);
        for _i in 0..3 {
            codice_fiscale.push(char::from_digit(digit.sample(&mut rng), 10).unwrap());
        }

        let value: String = codice_fiscale.iter().collect();
        let control_code = ControlCode::compute(&value);

        codice_fiscale.push(control_code);
        let codice_fiscale: String = codice_fiscale.iter().collect();
        GeneratorOutcome {
            codice_fiscale,
            omocodes: vec![],
        }
    }
}

impl GeneratorOutcome {
    pub fn get(&self) -> String {
        self.codice_fiscale.to_string()
    }

    pub fn omocodes(&self) -> Vec<String> {
        self.omocodes.to_vec()
    }
}

fn generate_omocodes(starting_codice_fiscale: &str) -> Vec<String> {
    let mut omocodes = Omocodes::generate(starting_codice_fiscale.chars());

    omocodes
        .iter_mut()
        .map(|omocode| {
            let control_code = ControlCode::compute(&omocode.iter().collect::<String>());
            omocode.push(control_code);
            omocode.iter().collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn generate_codice_fiscale(person_data: &PersonData) -> String {
    let surname_part = generate_name_or_surname_part(person_data.surname.to_owned());
    let name_part = generate_name_or_surname_part(person_data.name.to_owned());
    let birth_day_and_gender_parts =
        generate_birth_day_and_gender_parts(person_data.birthdate, person_data.gender);

    let mut codice_fiscale: String = [
        surname_part,
        name_part,
        birth_day_and_gender_parts,
        person_data.place_of_birth.chars().collect(),
    ]
    .concat()
    .iter()
    .collect();

    let control_code = ControlCode::compute(&codice_fiscale);
    codice_fiscale.push(control_code);

    codice_fiscale
}

fn generate_name_or_surname_part(name_or_surname: String) -> Vec<char> {
    let mut result = vec![];

    let mut name_consonants: Vec<char> = name_or_surname
        .chars()
        .filter(|char| !vowels().contains(char))
        .collect();
    result.append(&mut name_consonants);

    if result.len() >= 3 {
        return result[0..3].to_vec();
    }

    let mut name_vowels: Vec<char> = name_or_surname
        .chars()
        .filter(|char| vowels().contains(char))
        .collect();
    result.append(&mut name_vowels);

    if result.len() >= 3 {
        return result[0..3].to_vec();
    }

    while result.len() < 3 {
        result.push('X');
    }

    result
}

fn generate_birth_day_and_gender_parts(birthday: NaiveDate, gender: Gender) -> Vec<char> {
    let year_part: Vec<char> = birthday.year().to_string().chars().collect();
    let month_part = month_codes()[(birthday.month() as usize) - 1];

    let mut day: Vec<char> = if gender == Gender::F {
        (birthday.day() + 40).to_string().chars().collect()
    } else {
        birthday.day().to_string().chars().collect()
    };

    if day.len() < 2 {
        day.insert(0, '0');
    }

    [year_part[2..].to_vec(), vec![month_part], day].concat()
}

#[cfg(test)]
mod tests {
    use crate::verifier::Verifier;

    use super::*;

    #[test]
    fn generate_valid_random_codice_fiscale() {
        for _i in 0..10_000 {
            let codice_fiscale = Generator::generate_random();
            assert!(Verifier::verify(&codice_fiscale.get()).is_ok())
        }
    }

    #[test]
    fn generate_valid_codice_fiscale_from_person_data() {
        let person_data = PersonData {
            name: "PI".to_string(),
            surname: "SUCCHIO".to_string(),
            birthdate: NaiveDate::from_ymd_opt(1998, 7, 8).unwrap(),
            gender: Gender::F,
            place_of_birth: "M256".to_string(),
        };
        let codice_fiscale = Generator::generate(&person_data);

        assert_eq!(codice_fiscale.get(), "SCCPIX98L48M256N");
        assert!(Verifier::verify(&codice_fiscale.get()).is_ok());
    }
}
