use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ray_ray::math::Matrix;

fn matrix_mul_4() -> Matrix<4, 4> {
    let a = Matrix::new([
        [2.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);

    let b = Matrix::new([
        [3.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);
    a * b
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Matrix Multiply", |b| b.iter(|| matrix_mul_4()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
