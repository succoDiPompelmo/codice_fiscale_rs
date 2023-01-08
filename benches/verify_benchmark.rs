use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use codice_fiscale_rs::CodiceFiscale;

fn random_inputs(c: &mut Criterion) {
    let mut random_inputs = vec![];
    for _i in 0..3 {
        random_inputs.push(CodiceFiscale::generate_random(None))
    }

    let mut group = c.benchmark_group("verify_random_inputs");
    for codice_fiscale in random_inputs.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(codice_fiscale),
            codice_fiscale,
            |b, codice_fiscale| {
                b.iter(|| CodiceFiscale::verify(&codice_fiscale.get()));
            },
        );
    }
    group.finish();
}

fn single_input_verifier(c: &mut Criterion) {
    let codice_fiscale = "cTMTBT74E05B506W";

    c.bench_with_input(
        BenchmarkId::new("verify", codice_fiscale),
        &codice_fiscale,
        |b, input| {
            b.iter(|| CodiceFiscale::verify(input));
        },
    );
}

criterion_group!(benches, random_inputs, single_input_verifier);
criterion_main!(benches);
