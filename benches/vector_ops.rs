use criterion::{black_box, criterion_group, criterion_main, Criterion};
use manim_rs::core::Vector2D;

fn benchmark_vector_operations(c: &mut Criterion) {
    c.bench_function("vector_normalize_1000", |b| {
        let vectors: Vec<Vector2D> = (0..1000)
            .map(|i| Vector2D::new(i as f64, i as f64 + 1.0))
            .collect();

        b.iter(|| {
            for v in &vectors {
                black_box(v.normalize());
            }
        });
    });

    c.bench_function("vector_dot_product_1000", |b| {
        let v1 = Vector2D::new(3.0, 4.0);
        b.iter(|| {
            for i in 0..1000 {
                let v2 = Vector2D::new(i as f64, i as f64 + 1.0);
                black_box(v1.dot(v2));
            }
        });
    });

    c.bench_function("vector_lerp_1000", |b| {
        let start = Vector2D::new(0.0, 0.0);
        let end = Vector2D::new(100.0, 100.0);
        b.iter(|| {
            for i in 0..1000 {
                let t = i as f64 / 1000.0;
                black_box(start.lerp(end, t));
            }
        });
    });
}

criterion_group!(benches, benchmark_vector_operations);
criterion_main!(benches);

