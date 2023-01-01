use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use codice_fiscale_rs::CodiceFiscale;

fn from_elem(c: &mut Criterion) {
    let mut random_inputs = vec![];
    for _i in 0..5 {
        random_inputs.push(CodiceFiscale::generate_random())
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

criterion_group!(benches, from_elem);
criterion_main!(benches);
