//! # codice_fiscale_rs
//!
//! It aims to be a feature complete crate to generate and verify Italian codici fiscali.
//!
//! ## Overview
//!
//! The crate is based on two basics functionality:
//!
//! - Given a codice fiscale I must be able to tell if it's correct
//! - Given the personal data of an individual I must be able to generate the corresponding codice fiscale
//!
//! In addition for testing and verification purpose is possible to generate random codici fiscali.

use std::fmt;

use errors::VerifierError;
use generator::Generator;
use omocodes::Omocodes;
use person_data::PersonData;
use verifier::Verifier;

mod common;
mod control_code;
pub mod errors;
mod generator;
mod omocodes;
pub mod person_data;
mod verifier;

#[derive(Debug, PartialEq, Eq)]
pub struct CodiceFiscale {
    codice_fiscale: String,
}

impl fmt::Display for CodiceFiscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl CodiceFiscale {
    /// **Static** create a new CodiceFiscale struct with the provided string value.
    /// Returns an Ok result with CodiceFiscale struct as body if codice fiscale is valid,
    /// otherwise returns an error of type VerifierError.
    /// It detects the presence of omocode characters and considers it valid if the mapping,
    /// that you can find here <https://it.wikipedia.org/wiki/Omocodia> is satisfied.
    ///
    /// # Examples
    ///
    /// ```
    /// use codice_fiscale_rs::CodiceFiscale;
    ///
    /// let codice_fiscale_outcome = CodiceFiscale::new("BLTMHL77S04E889G");
    /// assert!(codice_fiscale_outcome.is_ok());
    /// ```
    ///
    /// ```
    /// use codice_fiscale_rs::CodiceFiscale;
    /// use codice_fiscale_rs::errors::VerifierError;
    ///
    /// let outcome = CodiceFiscale::new("BLTMHL77S04");
    /// assert_eq!(outcome, Err(VerifierError::InvalidLength(11)));
    /// ```
    ///
    /// ```
    /// use codice_fiscale_rs::CodiceFiscale;
    /// use codice_fiscale_rs::errors::VerifierError;
    ///
    /// let outcome = CodiceFiscale::new("BLTMHL77S04E889T");
    /// assert_eq!(outcome, Err(VerifierError::InvalidControlCharacter('T', 'G')));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the referenced fiscal code is not valid.
    pub fn new(raw_codice_fiscale: &str) -> Result<CodiceFiscale, VerifierError> {
        let codice_fiscale = CodiceFiscale {
            codice_fiscale: raw_codice_fiscale.to_string(),
        };

        Verifier::verify(&codice_fiscale.get())?;
        Ok(codice_fiscale)
    }

    /// **Static** method returns a CodiceFiscale struct from the personal data that
    /// is provided as input parameter. It computes the omocodes adn set them in the
    /// related field of the response struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use codice_fiscale_rs::CodiceFiscale;
    /// use codice_fiscale_rs::person_data::PersonData;;
    /// use codice_fiscale_rs::person_data::Gender;
    /// use chrono::{NaiveDate, Utc};
    ///
    /// let person_data = PersonData::new(
    ///     "PIPPO".to_string(),
    ///     "PLUTO".to_string(),
    ///     NaiveDate::from_ymd_opt(2023, 1, 7).unwrap(),
    ///     Gender::M,
    ///     "B544".to_string()).unwrap();
    ///
    /// let codice_fiscale_outcome = CodiceFiscale::generate(&person_data);
    /// assert_eq!(codice_fiscale_outcome.get(), "PLTPPP23A07B544K".to_string());
    /// ```
    pub fn generate(person_data: &PersonData) -> CodiceFiscale {
        let codice_fiscale = Generator::generate(person_data);
        CodiceFiscale { codice_fiscale }
    }

    /// Generate a random fiscal code. It's possible to provide a seed
    /// as an argument to generate predictable sequence of codici fiscali.
    pub fn generate_random(seed: Option<u64>) -> CodiceFiscale {
        let codice_fiscale = Generator::generate_random(seed);
        CodiceFiscale { codice_fiscale }
    }

    pub fn is_omocode(&self) -> bool {
        self.get() != Omocodes::replace_omocodes_characters(&self.get())
    }

    pub fn omocodes(&self) -> Vec<CodiceFiscale> {
        let omocodes = Generator::generate_omocodes(&self.get());
        omocodes
            .iter()
            .map(|cf| CodiceFiscale::new(cf).unwrap())
            .collect()
    }

    pub fn get(&self) -> String {
        self.codice_fiscale.to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::person_data::Gender;

    use super::*;

    #[test]
    fn test_display_trait() {
        let codice_fiscale = CodiceFiscale {
            codice_fiscale: "PLTPPP23A47T567Q".to_string(),
        };
        assert_eq!(format!("{}", codice_fiscale), "PLTPPP23A47T567Q");
    }

    #[test]
    fn test_verify() {
        assert!(CodiceFiscale::new("PLTPPP23A47T567Q").is_ok());
    }

    #[test]
    fn test_generate() {
        let naive_now = NaiveDate::from_ymd_opt(2022, 10, 2).unwrap();
        let person_data = PersonData::new(
            "PIPPO".to_string(),
            "PLUTO".to_string(),
            naive_now,
            Gender::F,
            "T567".to_string(),
        )
        .unwrap();

        let codice_fiscale = CodiceFiscale::generate(&person_data);

        assert_eq!(codice_fiscale.get(), "PLTPPP22R42T567K".to_string());
    }

    #[test]
    fn test_random_generator() {
        let codice_fiscale = CodiceFiscale::generate_random(Some(19));
        assert_eq!(codice_fiscale.get(), "ZLKESP25B55Y463L");
        assert!(CodiceFiscale::new(&codice_fiscale.get()).is_ok());
    }

    #[test]
    fn test_is_omocode_yes() {
        assert!(CodiceFiscale::new("BRNPRZ72D52F83VC").unwrap().is_omocode());
    }

    #[test]
    fn test_is_omocode_no() {
        assert!(!CodiceFiscale::new("ZLKESP25B55Y463L").unwrap().is_omocode());
    }

    #[test]
    fn omocodes_from_normal_cf() {
        let omocodes = CodiceFiscale::new("ZLKESP25B55Y463L").unwrap().omocodes();
        assert_eq!(omocodes.len(), 7);
        assert_eq!(omocodes.first().unwrap().get(), "ZLKESP25B55Y46PH");
    }

    #[test]
    fn omocodes_from_omocode_cf() {
        let omocodes = CodiceFiscale::new("BRNPRZ72D52F83VC").unwrap().omocodes();
        assert_eq!(omocodes.len(), 7);
        assert_eq!(omocodes.first().unwrap().get(), "BRNPRZ72D52F83VC");
    }
}
