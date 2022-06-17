use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/prom/mod.rs"] // Here
mod prom;

fn criterion_decode_labels(c: &mut Criterion) {
  c.bench_function("decode_labels", |b| b.iter(|| prom::parser::decode_labels(&String::from("key1=\"value1\",key2=\"0\",key3=\"value3\",key4=\"value4\""))));
}

fn criterion_decode_labels_with_rgx(c: &mut Criterion) {
  c.bench_function("decode_labels with rgx", |b| b.iter(|| prom::parser::decode_labels_with_rgx(&String::from("key1=\"value1\",key2=\"0\",key3=\"value3\",key4=\"value4\""))));
}

fn criterion_extract_labels(c: &mut Criterion) {
  c.bench_function("extract labels", |b| b.iter(|| prom::parser::extract_labels(&String::from("metric_1{key1=\"value1\",key2=\"0\",key3=\"value3\",key4=\"value4\"} 10.000007"))));
}

fn criterion_extract_labels_with_rgx(c: &mut Criterion) {
  c.bench_function("extract labels with rgx", |b| b.iter(|| prom::parser::extract_labels_with_rgx(&String::from("metric_1{key1=\"value1\",key2=\"0\",key3=\"value3\",key4=\"value4\"} 10.000007"))));
}

criterion_group!(benches, criterion_decode_labels, criterion_decode_labels_with_rgx, criterion_extract_labels, criterion_extract_labels_with_rgx);
criterion_main!(benches);