use chrono::NaiveDate;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use codice_fiscale_rs::{
    person_data::{Gender, PersonData},
    CodiceFiscale,
};

fn single_input_generator(c: &mut Criterion) {
    let person_data = PersonData::new(
        "GIOVANNI".to_string(),
        "CONTI".to_string(),
        NaiveDate::from_ymd_opt(2010, 10, 2).unwrap(),
        Gender::F,
        "Z111".to_string(),
    )
    .unwrap();

    c.bench_with_input(
        BenchmarkId::new("generate", person_data.clone()),
        &person_data,
        |b, input| {
            b.iter(|| CodiceFiscale::generate(input));
        },
    );
}

criterion_group!(benches, single_input_generator);
criterion_main!(benches);
