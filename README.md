# codice_fiscale_rs

![gitHub actions](https://github.com/succoDiPompelmo/codice_fiscale_rs/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/succoDiPompelmo/codice_fiscale_rs/branch/main/graph/badge.svg?token=IZCXZOETUS)](https://codecov.io/gh/succoDiPompelmo/codice_fiscale_rs)
[![crates.io][cratesio-image]][cratesio]

[cratesio-image]: https://img.shields.io/crates/v/codice_fiscale_rs.svg
[cratesio]: https://crates.io/crates/codice_fiscale_rs

Crate to manage Italian codice fiscale (tax code).

It aims to be a feature complete crate to generate and verify Italian codici fiscali.

## Limitations

**Special characters** are not handled correctly at the moment, they are considered invalid and an error will be returned. 
In the future, the government [Circolare N. 34/E](https://www.agenziaentrate.gov.it/portale/documents/20143/299856/Circolare+34+del+20+07+2011_circolare+34e.pdf/27b67cca-71db-9744-2ba4-6846460770e2) will be implemented.

**Omocode** feature is available but still needs some work to be considered complete.

**Codice catastale**, erroneously defined Befiore codes, are only accepted in input. The crate does not provide
any feature to translate a municipality name to the relative code. This will have a separate crate.