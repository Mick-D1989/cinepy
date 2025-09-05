use cine_core::{Video, exporters::FrameType};
use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

/// Benchmarks the performance of retrieving a frame from a video file.
fn get_frame_benchmark(c: &mut Criterion) {
    // when this gets run from the makefile, the crates root gets set as the working dir.
    let path = "../../files/temp.cine";
    if !std::path::Path::new(path).exists() {
        eprintln!(
            "\n[Benchmark Warning] The test file '{}' was not found.",
            path
        );
        return;
    }

    let mut group = c.benchmark_group("Get Frames As");
    group.sample_size(10);
    group.bench_function("get_frame_as_png", |b: &mut Bencher| {
        b.iter_batched(
            || {},
            |_| {
                let mut video_file =
                    Video::open(path).expect("Failed to open video file for benchmarking");
                let _ = video_file.get_frame_as(black_box(0), black_box(FrameType::Png));
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("get_frame_as_raw", |b: &mut Bencher| {
        b.iter_batched(
            || {},
            |_| {
                let mut video_file =
                    Video::open(path).expect("Failed to open video file for benchmarking");
                let _ = video_file.get_frame_as(black_box(0), black_box(FrameType::Raw));
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, get_frame_benchmark);
criterion_main!(benches);
