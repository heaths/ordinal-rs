// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ordinal_trait::Ordinal;

const INPUTS: &[usize] = &[1, 12, usize::MAX];

pub fn fmt(c: &mut Criterion) {
    let mut group = c.benchmark_group("fmt");
    for i in INPUTS {
        group.bench_with_input(BenchmarkId::new("ordinal-trait", i), i, |b, i| {
            b.iter(|| black_box(format!("{}", i.to_number())))
        });
        group.bench_with_input(BenchmarkId::new("num-ordinal", i), i, |b, i| {
            // cspell:ignore Osize
            b.iter(|| {
                black_box(format!(
                    "{}",
                    num_ordinal::ordinal1::<num_ordinal::Osize>(*i)
                ))
            })
        });
        group.bench_with_input(BenchmarkId::new("ordinal", i), i, |b, i| {
            b.iter(|| black_box(format!("{}", ordinal::Ordinal(*i))))
        });
        group.bench_with_input(BenchmarkId::new("ordinal-type", i), i, |b, i| {
            b.iter(|| black_box(format!("{}", ordinal_type::Ordinal(*i))))
        });
    }
    group.finish();
}

pub fn suffix(c: &mut Criterion) {
    let mut group = c.benchmark_group("suffix");
    for i in INPUTS {
        group.bench_with_input(BenchmarkId::new("ordinal-trait", i), i, |b, i| {
            b.iter(|| black_box(i.suffix()))
        });
        group.bench_with_input(BenchmarkId::new("ordinal", i), i, |b, i| {
            b.iter(|| black_box(ordinal::Ordinal(*i).suffix()))
        });
        group.bench_with_input(BenchmarkId::new("ordinal-type", i), i, |b, i| {
            b.iter(|| black_box(ordinal_type::Ordinal(*i).suffix()))
        });
    }
    group.finish();
}

criterion_group!(ordinal, fmt, suffix);
criterion_main!(ordinal);
