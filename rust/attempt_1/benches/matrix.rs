use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ray_ray::math::Matrix;

fn matrix_mul_4() -> Matrix {
    let a = Matrix::from([
        [2.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    let b = Matrix::from([
        [3.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);
    a * b
}

fn create_matrix_4() -> Matrix {
    let a = Matrix::from([
        [2.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);
    a
}

fn transpose_matrix_4() -> Matrix {
    let a = Matrix::from([
        [2.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);
    a.transpose()
}

fn test_matrix_det_4() -> f64 {
    let a = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);
    a.determinate()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Matrix Multiply", |b| b.iter(|| matrix_mul_4()));
    c.bench_function("Create Matrix", |b| b.iter(|| create_matrix_4()));
    c.bench_function("Transpose Matrix", |b| b.iter(|| transpose_matrix_4()));
    c.bench_function("Matrix Determinate", |b| b.iter(|| test_matrix_det_4()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
