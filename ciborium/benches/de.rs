use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_cbor_de(c: &mut Criterion) {
    let cases = ["trivial_helloworld", "twitter.json", "citm_catalog.json"];

    for name in cases {
        let data = fs::read(format!("../dag-cbor-benchmark/data/{name}.dagcbor")).unwrap();
        c.bench_function(&format!("cbor-de-{name}"), |b| {
            b.iter(|| {
                ciborium::de::from_reader::<serde::de::IgnoredAny, _>(&mut &data[..]).unwrap()
            });
        });
    }
}

criterion_group!(benches, bench_cbor_de);
criterion_main!(benches);
