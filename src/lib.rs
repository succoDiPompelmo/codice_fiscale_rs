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
    pub fn verify(codice_fiscale: &str) -> Result<CodiceFiscale, VerifierError> {
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
