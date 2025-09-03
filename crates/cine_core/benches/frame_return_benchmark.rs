use cine_core::{Video, exporters::FrameType, file::VideoOps};
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

    // --- Get Frames As Benchmark Group ---
    let mut group = c.benchmark_group("Get Frames As");

    // It's often useful to set a sample size for more stable results.
    group.sample_size(10);

    // Benchmark retrieving a frame and converting it to a PNG.
    group.bench_function("get_frame_as_png", |b: &mut Bencher| {
        // `iter_batched` is perfect for this scenario. It separates the 'setup'
        // (like opening a file) from the 'routine' that you actually want to measure.
        b.iter_batched(
            // SETUP: This closure is run before each measurement, but its execution time is not measured.
            // It prepares the state needed for the benchmarked routine.
            || Video::open(path).expect("Failed to open video file for benchmarking"),
            // ROUTINE: This is the code that will be benchmarked.
            // It receives the output of the setup closure (the opened file).
            |mut video_file| {
                let _ = video_file.get_frame_as(black_box(0), black_box(FrameType::Png));
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // Benchmark retrieving a frame as raw pixel data.
    // This provides a good comparison against the PNG conversion.
    group.bench_function("get_frame_as_raw", |b: &mut Bencher| {
        b.iter_batched(
            || Video::open(path).expect("Failed to open video file for benchmarking"),
            |mut video_file| {
                let _ = video_file.get_frame_as(black_box(0), black_box(FrameType::Raw));
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

// These macros register the benchmark function with Criterion's test harness.
criterion_group!(benches, get_frame_benchmark);
criterion_main!(benches);
