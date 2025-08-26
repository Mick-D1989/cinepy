use cine_py::file::CineFile;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use pyo3::prelude::*;
use rand::Rng;

fn get_frame_benchmark(c: &mut Criterion) {
    // when this gets run from the makefile, the crates root gets set as the working dir.
    let mut cine_file: CineFile = CineFile::new("./files/temp.cine");

    c.bench_function("get_random_frame", |b| {
        b.iter_batched(
            // Setup closure: runs before each measurement.
            || {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..400)
            },
            // Measurement closure.
            |frame_index| cine_file.get_frame(black_box(frame_index)),
            criterion::BatchSize::SmallInput,
        );
    });
}

fn get_b64_benchmark(c: &mut Criterion) {
    // when this gets run from the makefile, the crates root gets set as the working dir.
    let mut cine_file: CineFile = CineFile::new("./files/temp.cine");

    c.bench_function("get_b64_frame", |b| {
        b.iter_batched(
            // Setup closure: runs before each measurement.
            || {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..400)
            },
            // Measurement closure.
            |frame_index| cine_file.base64_png(black_box(frame_index)),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, get_frame_benchmark, get_b64_benchmark);
criterion_main!(benches);
