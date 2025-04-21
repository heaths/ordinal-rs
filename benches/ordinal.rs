// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{
    black_box, criterion_group, criterion_main, profiler::Profiler, BenchmarkId, Criterion,
};
use num_format::{Locale, ToFormattedString as _};
use ordinal::ToOrdinal;
use std::{
    alloc::{GlobalAlloc, System},
    sync::atomic::{AtomicUsize, Ordering},
};

const INPUTS: &[usize] = &[1, 12, usize::MAX];

#[global_allocator]
static GLOBAL: ReportingAllocator<System> = ReportingAllocator::new(System);

pub fn fmt(c: &mut Criterion) {
    let mut group = c.benchmark_group("fmt");
    for i in INPUTS {
        group.bench_with_input(BenchmarkId::new("ordinal", i), i, |b, i| {
            b.iter(|| black_box(i.to_ordinal_string()))
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
        group.bench_with_input(BenchmarkId::new("ordinal@0.3.2", i), i, |b, i| {
            b.iter(|| black_box(format!("{}", ordinal_legacy::Ordinal(*i))))
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
        group.bench_with_input(BenchmarkId::new("ordinal", i), i, |b, i| {
            b.iter(|| black_box(i.suffix()))
        });
        group.bench_with_input(BenchmarkId::new("ordinal@0.3.2", i), i, |b, i| {
            b.iter(|| black_box(ordinal_legacy::Ordinal(*i).suffix()))
        });
        group.bench_with_input(BenchmarkId::new("ordinal-type", i), i, |b, i| {
            b.iter(|| black_box(ordinal_type::Ordinal(*i).suffix()))
        });
    }
    group.finish();
}

criterion_group! {
    name = ordinal;
    config = Criterion::default().with_profiler(MemoryProfiler);
    targets = fmt, suffix
}
criterion_main!(ordinal);

struct ReportingAllocator<T: GlobalAlloc> {
    alloc: T,
    size: AtomicUsize,
}

impl<T: GlobalAlloc> ReportingAllocator<T> {
    pub const fn new(alloc: T) -> Self {
        Self {
            alloc,
            size: AtomicUsize::new(0),
        }
    }

    pub fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }

    pub fn reset(&self) {
        self.size.store(0, Ordering::SeqCst);
    }
}

unsafe impl<T: GlobalAlloc> GlobalAlloc for ReportingAllocator<T> {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        self.size.fetch_add(layout.size(), Ordering::SeqCst);
        self.alloc.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        self.alloc.dealloc(ptr, layout);
    }
}

struct MemoryProfiler;

impl Profiler for MemoryProfiler {
    fn start_profiling(&mut self, _: &str, _: &std::path::Path) {
        GLOBAL.reset();
    }

    fn stop_profiling(&mut self, _: &str, _: &std::path::Path) {
        let size = GLOBAL.size() / 1024;
        println!("; allocated {} KiB", size.to_formatted_string(&Locale::en));
    }
}
