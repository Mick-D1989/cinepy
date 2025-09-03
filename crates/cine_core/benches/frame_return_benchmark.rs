use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
// TODO: Replace 'frame_decoder' with the actual name of your crate from Cargo.toml.
use cine_core::{Video, exporters::FrameType, file::VideoOps};

/// Benchmarks the performance of retrieving a frame from a video file.
fn get_frame_benchmark(c: &mut Criterion) {
    // NOTE: This benchmark requires a real '.cine' file to run.
    // Please place a sample file named 'test.cine' in the root of your project directory.
    let path = "../../files/temp.cine";

    // We check if the test file exists to provide a more helpful message if it's missing.
    if !std::path::Path::new(path).exists() {
        eprintln!(
            "\n[Benchmark Warning] The test file '{}' was not found. \
            Please place a valid .cine file in the project root to run this benchmark.\n",
            path
        );
        return;
    }

    // --- Benchmark Group ---
    // We can group related benchmarks together for clearer output.
    let mut group = c.benchmark_group("Frame Retrieval");

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
                // We use `black_box` to ensure the compiler doesn't optimize away the function call
                // or its parameters, which could lead to inaccurate measurements.
                let _ = video_file.get_frame_as(black_box(0), black_box(FrameType::Png));
            },
            // BATCH SIZE: This determines how many times the routine is run per setup.
            // For operations that might change state (like reading from a file),
            // a small batch size is appropriate to ensure each run is independent.
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
