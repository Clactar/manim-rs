//! Benchmarks for path operations to ensure performance targets are met.
//!
//! Target: Path creation < 100ns for small shapes (stack-allocated via SmallVec).

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use manim_rs::core::{Transform, Vector2D};
use manim_rs::renderer::Path;

/// Benchmark creating a simple triangle (5 commands - should be stack-allocated).
fn bench_triangle_creation(c: &mut Criterion) {
    c.bench_function("triangle_creation", |b| {
        b.iter(|| {
            let mut path = Path::new();
            path.move_to(black_box(Vector2D::new(0.0, 0.0)))
                .line_to(black_box(Vector2D::new(1.0, 0.0)))
                .line_to(black_box(Vector2D::new(0.5, 1.0)))
                .close();
            black_box(path)
        });
    });
}

/// Benchmark creating a square (5 commands - should be stack-allocated).
fn bench_square_creation(c: &mut Criterion) {
    c.bench_function("square_creation", |b| {
        b.iter(|| {
            let mut path = Path::new();
            path.move_to(black_box(Vector2D::new(0.0, 0.0)))
                .line_to(black_box(Vector2D::new(1.0, 0.0)))
                .line_to(black_box(Vector2D::new(1.0, 1.0)))
                .line_to(black_box(Vector2D::new(0.0, 1.0)))
                .close();
            black_box(path)
        });
    });
}

/// Benchmark creating a circle approximation (13 commands - should be stack-allocated).
fn bench_circle_creation(c: &mut Criterion) {
    c.bench_function("circle_creation", |b| {
        b.iter(|| {
            let mut path = Path::new();
            let radius = black_box(2.0);
            let magic = 0.551_915_024_493_510_6;

            path.move_to(Vector2D::new(radius, 0.0));

            // 4 cubic bezier curves to approximate a circle
            path.cubic_to(
                Vector2D::new(radius, radius * magic),
                Vector2D::new(radius * magic, radius),
                Vector2D::new(0.0, radius),
            );
            path.cubic_to(
                Vector2D::new(-radius * magic, radius),
                Vector2D::new(-radius, radius * magic),
                Vector2D::new(-radius, 0.0),
            );
            path.cubic_to(
                Vector2D::new(-radius, -radius * magic),
                Vector2D::new(-radius * magic, -radius),
                Vector2D::new(0.0, -radius),
            );
            path.cubic_to(
                Vector2D::new(radius * magic, -radius),
                Vector2D::new(radius, -radius * magic),
                Vector2D::new(radius, 0.0),
            );
            path.close();

            black_box(path)
        });
    });
}

/// Benchmark creating a large path (100 commands - heap allocation).
fn bench_large_path_creation(c: &mut Criterion) {
    c.bench_function("large_path_creation", |b| {
        b.iter(|| {
            let mut path = Path::with_capacity(100);
            path.move_to(Vector2D::new(0.0, 0.0));
            for i in 1..100 {
                let x = (i as f64) * 0.1;
                let y = (i as f64 * 0.05).sin();
                path.line_to(black_box(Vector2D::new(x, y)));
            }
            black_box(path)
        });
    });
}

/// Benchmark bounding box computation.
fn bench_bounding_box(c: &mut Criterion) {
    let mut group = c.benchmark_group("bounding_box");

    // Small path (triangle)
    let mut small_path = Path::new();
    small_path
        .move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 0.0))
        .line_to(Vector2D::new(0.5, 1.0))
        .close();

    group.bench_function("small_path", |b| {
        b.iter(|| {
            let bbox = small_path.bounding_box();
            black_box(bbox)
        });
    });

    // Large path
    let mut large_path = Path::with_capacity(100);
    large_path.move_to(Vector2D::new(0.0, 0.0));
    for i in 1..100 {
        large_path.line_to(Vector2D::new((i as f64) * 0.1, (i as f64).sin()));
    }

    group.bench_function("large_path", |b| {
        b.iter(|| {
            let bbox = large_path.bounding_box();
            black_box(bbox)
        });
    });

    group.finish();
}

/// Benchmark bounding box caching.
fn bench_bounding_box_cached(c: &mut Criterion) {
    let mut path = Path::new();
    path.move_to(Vector2D::new(0.0, 0.0))
        .line_to(Vector2D::new(1.0, 0.0))
        .line_to(Vector2D::new(0.5, 1.0))
        .close();

    // Pre-compute to populate cache
    let _ = path.bounding_box();

    c.bench_function("bounding_box_cached", |b| {
        b.iter(|| {
            let bbox = path.bounding_box();
            black_box(bbox)
        });
    });
}

/// Benchmark transform application.
fn bench_transform_application(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform");

    let transform = Transform::translate(2.0, 3.0);

    // Small path
    group.bench_function("small_path", |b| {
        b.iter(|| {
            let mut path = Path::new();
            path.move_to(Vector2D::new(0.0, 0.0))
                .line_to(Vector2D::new(1.0, 0.0))
                .line_to(Vector2D::new(0.5, 1.0))
                .close();
            path.apply_transform(black_box(&transform));
            black_box(path)
        });
    });

    // Large path
    group.bench_function("large_path", |b| {
        b.iter(|| {
            let mut path = Path::with_capacity(100);
            path.move_to(Vector2D::new(0.0, 0.0));
            for i in 1..100 {
                path.line_to(Vector2D::new((i as f64) * 0.1, (i as f64).sin()));
            }
            path.apply_transform(black_box(&transform));
            black_box(path)
        });
    });

    group.finish();
}

/// Benchmark path cloning.
fn bench_path_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone");

    // Small path (stack-allocated)
    let small_path = {
        let mut p = Path::new();
        p.move_to(Vector2D::new(0.0, 0.0))
            .line_to(Vector2D::new(1.0, 0.0))
            .line_to(Vector2D::new(0.5, 1.0))
            .close();
        p
    };

    group.bench_function("small_path", |b| {
        b.iter(|| {
            let cloned = small_path.clone();
            black_box(cloned)
        });
    });

    // Large path (heap-allocated)
    let large_path = {
        let mut p = Path::with_capacity(100);
        p.move_to(Vector2D::new(0.0, 0.0));
        for i in 1..100 {
            p.line_to(Vector2D::new((i as f64) * 0.1, (i as f64).sin()));
        }
        p
    };

    group.bench_function("large_path", |b| {
        b.iter(|| {
            let cloned = large_path.clone();
            black_box(cloned)
        });
    });

    group.finish();
}

/// Benchmark various path sizes to see SmallVec optimization threshold.
fn bench_path_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_size_scaling");

    for size in [5, 10, 16, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut path = Path::new();
                path.move_to(Vector2D::new(0.0, 0.0));
                for i in 1..size {
                    path.line_to(Vector2D::new((i as f64) * 0.1, (i as f64).sin()));
                }
                black_box(path)
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_triangle_creation,
    bench_square_creation,
    bench_circle_creation,
    bench_large_path_creation,
    bench_bounding_box,
    bench_bounding_box_cached,
    bench_transform_application,
    bench_path_clone,
    bench_path_sizes
);
criterion_main!(benches);
