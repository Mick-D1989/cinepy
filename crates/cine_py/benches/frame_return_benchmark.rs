use cine_py::file::CineFile;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn get_frame_benchmark(c: &mut Criterion) {
    let mut cine_file = CineFile::new("/mnt/c/Users/micha/projects/cinepy/files/temp.cine");

    c.bench_function("get_random_frame", |b| {
        b.iter_batched(
            // Setup closure: runs before each measurement.
            || {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..7400)
            },
            // Measurement closure.
            |frame_index| cine_file.get_frame(black_box(frame_index)),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, get_frame_benchmark);
criterion_main!(benches);
