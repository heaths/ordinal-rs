// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ordinal::Ordinal;

pub fn suffix(c: &mut Criterion) {
    let mut group = c.benchmark_group("suffix");
    for i in [1usize, 2, 3, 12, 123_456_789] {
        group.bench_with_input(BenchmarkId::new("numeric_modulo", i), &i, |b, i| {
            b.iter(|| i.suffix())
        });
        group.bench_with_input(BenchmarkId::new("string_ends_with", i), &i, |b, i| {
            b.iter(|| suffix_ends_with(*i))
        });
        group.bench_with_input(BenchmarkId::new("string_i8_modulo", i), &i, |b, i| {
            b.iter(|| suffix_i8_modulo(*i))
        });
    }
    group.finish();
}

criterion_group!(ordinal, suffix);
criterion_main!(ordinal);

fn suffix_ends_with(i: usize) -> &'static str {
    let s = i.to_string();
    if s.ends_with("1") && !s.ends_with("11") {
        "st"
    } else if s.ends_with("2") && !s.ends_with("12") {
        "nd"
    } else if s.ends_with("3") && !s.ends_with("13") {
        "rd"
    } else {
        "th"
    }
}

fn suffix_i8_modulo(i: usize) -> &'static str {
    use std::str::FromStr;

    let str_repr = i.to_string();

    let len = str_repr.len();
    let last_two_chars = if len > 1 {
        str_repr.get(len - 2..).unwrap_or(&str_repr)
    } else {
        &str_repr
    };

    let ordinal_num = i8::from_str(last_two_chars)
        .map(i8::abs)
        .unwrap_or_default();

    match ordinal_num % 100 {
        11..=13 => "th",
        _ => match ordinal_num % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}
