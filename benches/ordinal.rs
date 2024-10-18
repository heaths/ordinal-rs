// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ordinal::Ordinal;

const INPUTS: &[usize] = &[1usize, 2, 3, 12, 123_456_789];

pub fn fmt(c: &mut Criterion) {
    let mut group = c.benchmark_group("fmt");
    for i in INPUTS {
        group.bench_with_input(BenchmarkId::new("ordinal", i), i, |b, i| {
            b.iter(|| black_box(i.ordinal()))
        });
        group.bench_with_input(BenchmarkId::new("string_match", i), i, |b, i| {
            b.iter(|| black_box(ordinal_string_match(*i)))
        });
    }
    group.finish();
}

pub fn suffix(c: &mut Criterion) {
    let mut group = c.benchmark_group("suffix");
    for i in INPUTS {
        group.bench_with_input(BenchmarkId::new("numeric_modulo", i), i, |b, i| {
            b.iter(|| black_box(i.suffix()))
        });
        group.bench_with_input(BenchmarkId::new("string_ends_with", i), i, |b, i| {
            b.iter(|| black_box(suffix_ends_with(*i)))
        });
        group.bench_with_input(BenchmarkId::new("string_i8_modulo", i), i, |b, i| {
            b.iter(|| black_box(suffix_i8_modulo(*i)))
        });
    }
    group.finish();
}

criterion_group!(ordinal, fmt, suffix);
criterion_main!(ordinal);

fn ordinal_string_match(i: usize) -> String {
    let s = i.to_string();
    let suffix = match s.bytes().last().unwrap() {
        b'1' if !s.ends_with("11") => "st",
        b'2' if !s.ends_with("12") => "nd",
        b'3' if !s.ends_with("13") => "rd",
        _ => "th",
    };
    format!("{}{}", s, suffix)
}

fn suffix_ends_with(i: usize) -> &'static str {
    // https://github.com/gleich/ordinal/blob/a4bf9bdc37d05940f87d8ceea1c4b47cda0da5b4/src/lib.rs#L59-L70
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
    // https://github.com/gleich/ordinal/blob/fbcd2bece0ad81ef1c2da695c8e70bd34be94622/src/lib.rs#L59-L86
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
