use std::fmt;

use chrono::prelude::*;
use generator::Generator;
use thiserror::Error;
use verifier::{Verifier, VerifierError};

mod common;
mod control_code;
mod generator;
mod omocodes;
mod verifier;

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

type Result<T> = std::result::Result<T, CodiceFiscaleError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CodiceFiscaleError {
    #[error(transparent)]
    VerifyError(#[from] VerifierError),
}

impl CodiceFiscale {
    pub fn verify(codice_fiscale: &str) -> Result<CodiceFiscale> {
        let verifier_outcome = Verifier::verify(codice_fiscale)?;

        Ok(CodiceFiscale {
            codice_fiscale: verifier_outcome.get(),
            omocodes: vec![],
        })
    }

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
}
