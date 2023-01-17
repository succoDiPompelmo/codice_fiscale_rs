use std::vec;

use chrono::prelude::*;
use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};

use crate::{
    common,
    control_code::ControlCode,
    omocodes::Omocodes,
    person_data::{Gender, PersonData},
};

pub struct Generator {}

impl Generator {
    pub fn generate(person_data: &PersonData) -> String {
        let surname_part = generate_name_or_surname_part(person_data.surname());
        let name_part = generate_name_or_surname_part(person_data.name());
        let birth_day_and_gender_part =
            generate_birth_day_and_gender_parts(person_data.birthdate(), person_data.gender());

        let mut codice_fiscale: String = [
            surname_part,
            name_part,
            birth_day_and_gender_part,
            person_data.birth_place().chars().collect(),
        ]
        .concat()
        .iter()
        .collect();

        codice_fiscale.push(ControlCode::compute(&codice_fiscale));
        codice_fiscale
    }

    pub fn generate_omocodes(starting_codice_fiscale: &str) -> Vec<String> {
        let mut omocodes = Omocodes::generate(
            Omocodes::replace_omocodes_characters(starting_codice_fiscale).chars(),
        );

        omocodes
            .iter_mut()
            .map(|omocode| {
                let control_code = ControlCode::compute(&omocode.iter().collect::<String>());
                if omocode.len() == 16 {
                    omocode.pop();
                }
                omocode.push(control_code);
                omocode.iter().collect::<String>()
            })
            .collect::<Vec<String>>()
    }

    pub fn generate_random(seed: Option<u64>) -> String {
        let mut codice_fiscale = vec![];
        let mut rng = seed.map_or(StdRng::from_entropy(), StdRng::seed_from_u64);

        let alphabet_index = Uniform::from(1..26);
        let digit = Uniform::from(0..10);

        for _i in 0..6 {
            codice_fiscale.push(common::to_alphabet(alphabet_index.sample(&mut rng)))
        }

        for _i in 0..2 {
            codice_fiscale.push(char::from_digit(digit.sample(&mut rng), 10).unwrap());
        }

        codice_fiscale.push(common::to_month_codes(
            Uniform::from(0..12).sample(&mut rng),
        ));

        codice_fiscale.push(
            char::from_digit(
                Uniform::from(1..2).sample(&mut rng) + 4 * Uniform::from(0..2).sample(&mut rng),
                10,
            )
            .unwrap(),
        );

        codice_fiscale.push(char::from_digit(Uniform::from(0..10).sample(&mut rng), 10).unwrap());
        codice_fiscale.push(common::to_alphabet(alphabet_index.sample(&mut rng)));
        for _i in 0..3 {
            codice_fiscale.push(char::from_digit(digit.sample(&mut rng), 10).unwrap());
        }

        let value: String = codice_fiscale.iter().collect();
        let control_code = ControlCode::compute(&value);

        codice_fiscale.push(control_code);
        codice_fiscale.iter().collect()
    }
}

fn generate_name_or_surname_part(value: String) -> Vec<char> {
    let mut result = vec![];

    result.append(&mut value.chars().filter(common::is_consonant).collect());
    if result.len() >= 3 {
        return result[0..3].to_vec();
    }

    result.append(&mut value.chars().filter(common::is_vowel).collect());
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
    let month_part = common::to_month_codes((birthday.month() as usize) - 1);

    let mut day_part: Vec<char> = match gender {
        Gender::F => (birthday.day() + 40).to_string().chars().collect(),
        Gender::M => birthday.day().to_string().chars().collect(),
    };

    if day_part.len() < 2 {
        day_part.insert(0, '0');
    }

    [year_part[2..].to_vec(), vec![month_part], day_part].concat()
}

#[cfg(test)]
mod tests {
    use crate::CodiceFiscale;

    use super::*;

    #[test]
    fn generate_valid_random_codice_fiscale() {
        for _i in 0..10_000 {
            let codice_fiscale = Generator::generate_random(None);
            assert!(CodiceFiscale::new(&codice_fiscale).is_ok());
        }
    }

    #[test]
    fn generate_valid_codice_fiscale_from_person_data_female() {
        let person_data = PersonData::new(
            "PI".to_string(),
            "SUCCHIO".to_string(),
            NaiveDate::from_ymd_opt(1998, 7, 8).unwrap(),
            Gender::F,
            "M256".to_string(),
        )
        .unwrap();
        let codice_fiscale = Generator::generate(&person_data);

        assert_eq!(codice_fiscale, "SCCPIX98L48M256N");
        assert!(CodiceFiscale::new(&codice_fiscale).is_ok());
    }

    #[test]
    fn generate_valid_codice_fiscale_from_person_data_male() {
        let person_data = PersonData::new(
            "PI".to_string(),
            "SUCCHIO".to_string(),
            NaiveDate::from_ymd_opt(1998, 7, 8).unwrap(),
            Gender::M,
            "M256".to_string(),
        )
        .unwrap();
        let codice_fiscale = Generator::generate(&person_data);

        assert_eq!(codice_fiscale, "SCCPIX98L08M256J");
        assert!(CodiceFiscale::new(&codice_fiscale).is_ok());
    }
}
