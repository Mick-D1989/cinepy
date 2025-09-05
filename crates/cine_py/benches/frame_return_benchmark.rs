use cine_py::wrappers::CinePy;

use criterion::{Bencher, Criterion, criterion_group, criterion_main};

use pyo3::prelude::*;
use pyo3::types::PyModule;

use rand::Rng;

use std::hint::black_box;

fn rust_py_wrapper_benchmark(c: &mut Criterion) {
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
    let mut rust_group = c.benchmark_group("Rust Get Frames As");

    rust_group.sample_size(10);

    // Benchmark retrieving a frame and converting it to a PNG.
    rust_group.bench_function("rust_get_frame_as_png", |b: &mut Bencher| {
        b.iter_batched(
            || (),
            |_| {
                let mut video_file =
                    CinePy::new(path).expect("Failed to open video for benchmarking");
                let _ = video_file
                    .get_frame_as(black_box(0), black_box(cine_py::wrappers::PyFrameType::Png));
            },
            criterion::BatchSize::SmallInput,
        );
    });

    rust_group.finish();
}

fn py_bindings_benchmark(c: &mut Criterion) {
    // when this gets run from the makefile, the crates root gets set as the working dir.
    let path = "../../files/temp.cine";
    if !std::path::Path::new(path).exists() {
        eprintln!(
            "\n[Benchmark Warning] The test file '{}' was not found.",
            path
        );
        return;
    }

    let mut py_group = c.benchmark_group("Python Get Frames As");
    py_group.sample_size(10);
    py_group.bench_function("python_get_frame_as_png", |b: &mut Bencher| {
        b.iter_batched(
            || {
                let pyclass: Py<PyAny> = Python::with_gil(|py| {
                    let cine_py_module =
                        PyModule::import(py, "cine_py").expect("Unable to import cine_py");
                    let cine_py_class = cine_py_module
                        .getattr("CinePy")
                        .expect("Failed to get CinePy class");
                    cine_py_class.into()
                });
                let owned_path = path.to_string();
                (pyclass, owned_path)
            },
            |(cine_py_class_obj, o_path)| {
                Python::with_gil(|py| {
                    let cine_py_class: &Py<PyAny> = cine_py_class_obj.as_ref();

                    // Open file
                    let cine_file_instance = cine_py_class
                        .call1(py, (o_path,))
                        .expect("Failed to create CinePy python instance");

                    // Get the frame
                    let _ = black_box(
                        cine_file_instance
                            .call_method1(
                                py,
                                "get_frame_as",
                                (black_box(0), black_box("PyFrameType::Png")),
                            )
                            .expect("Python method call failed"),
                    );
                })
            },
            criterion::BatchSize::SmallInput,
        );
    });
    py_group.finish();
}

criterion_group!(benches, rust_py_wrapper_benchmark);
criterion_main!(benches);
