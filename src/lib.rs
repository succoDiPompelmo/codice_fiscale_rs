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
    omocodes: Vec<String>,
}

impl fmt::Display for CodiceFiscale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl CodiceFiscale {
    /// **Static** method returns an Ok result with CodiceFiscale struct as body if codice fiscale is valid,
    /// otherwise returns an error of type VerifierError.
    /// It detects the presence of omocode characters and considers it valid if the mapping,
    /// that you can find here <https://it.wikipedia.org/wiki/Omocodia>, is correct.
    ///
    /// # Examples
    ///
    /// ```
    /// use codice_fiscale_rs::CodiceFiscale;
    ///
    /// let codice_fiscale_outcome = CodiceFiscale::verify("BLTMHL77S04E889G");
    /// assert!(codice_fiscale_outcome.is_ok());
    /// ```
    ///
    /// ```
    /// use codice_fiscale_rs::CodiceFiscale;
    /// use codice_fiscale_rs::errors::VerifierError;
    ///
    /// let codice_fiscale_outcome = CodiceFiscale::verify("BLTMHL77S04");
    /// assert_eq!(codice_fiscale_outcome, Err(VerifierError::InvalidLength(11)));
    /// ```
    ///
    /// ```
    /// use codice_fiscale_rs::CodiceFiscale;
    /// use codice_fiscale_rs::errors::VerifierError;
    ///
    /// let codice_fiscale_outcome = CodiceFiscale::verify("BLTMHL77S04E889T");
    /// assert_eq!(codice_fiscale_outcome, Err(VerifierError::InvalidControlCharacter('T', 'G')));
    /// ```
    pub fn verify(codice_fiscale: &str) -> Result<CodiceFiscale, VerifierError> {
        let verifier_outcome = Verifier::verify(codice_fiscale)?;

        Ok(CodiceFiscale {
            codice_fiscale: verifier_outcome.get(),
            omocodes: vec![],
        })
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
    ///     Utc::now().date_naive(),
    ///     Gender::M,
    ///     "B544".to_string()).unwrap();
    ///
    /// let codice_fiscale_outcome = CodiceFiscale::generate(&person_data);
    /// assert_eq!(codice_fiscale_outcome.get(), "PLTPPP23A07B544K".to_string());
    ///
    /// let omocodes = codice_fiscale_outcome.omocodes();
    /// assert_eq!(omocodes.len(), 7);
    /// assert_eq!(omocodes.last(), Some(&"PLTPPPNPALTBRQQKX".to_string()));
    /// ```
    pub fn generate(person_data: &PersonData) -> CodiceFiscale {
        let generator_outcome = Generator::generate(person_data);

        CodiceFiscale {
            codice_fiscale: generator_outcome.get(),
            omocodes: generator_outcome.omocodes(),
        }
    }

    pub fn generate_random() -> CodiceFiscale {
        let generator_outcome = Generator::generate_random();
        CodiceFiscale {
            codice_fiscale: generator_outcome.get(),
            omocodes: vec![],
        }
    }

    pub fn get(&self) -> String {
        self.codice_fiscale.to_string()
    }

    pub fn omocodes(&self) -> Vec<String> {
        self.omocodes.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::person_data::Gender;

    use super::*;

    #[test]
    fn test_display_trait() {
        let codice_fiscale = CodiceFiscale {
            codice_fiscale: "PLTPPP23A47T567Q".to_string(),
            omocodes: vec![],
        };
        assert_eq!(format!("{}", codice_fiscale), "PLTPPP23A47T567Q");
    }

    #[test]
    fn test_verify() {
        let codice_fiscale = CodiceFiscale::verify("PLTPPP23A47T567Q");
        assert!(codice_fiscale.is_ok());
    }

    #[test]
    fn test_generate() {
        let naive_now = Utc::now().date_naive();
        let person_data = PersonData::new(
            "PIPPO".to_string(),
            "PLUTO".to_string(),
            naive_now,
            Gender::F,
            "T567".to_string(),
        )
        .unwrap();

        let codice_fiscale = CodiceFiscale::generate(&person_data);

        assert_eq!(codice_fiscale.get(), "PLTPPP23A47T567Q".to_string());
    }
}
