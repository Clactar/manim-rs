//! Benchmarks for mobject operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use manim_rs::core::{Color, Vector2D};
use manim_rs::mobject::geometry::{Circle, Polygon, Rectangle};
use manim_rs::mobject::{Mobject, MobjectGroup};

fn bench_circle_creation(c: &mut Criterion) {
    c.bench_function("circle_creation", |b| {
        b.iter(|| {
            let _circle = Circle::new(black_box(1.0));
        });
    });
}

fn bench_circle_builder(c: &mut Criterion) {
    c.bench_function("circle_builder", |b| {
        b.iter(|| {
            let _circle = Circle::builder()
                .radius(black_box(2.0))
                .stroke_color(Color::BLUE)
                .fill_color(Color::RED)
                .build();
        });
    });
}

fn bench_rectangle_creation(c: &mut Criterion) {
    c.bench_function("rectangle_creation", |b| {
        b.iter(|| {
            let _rect = Rectangle::new(black_box(2.0), black_box(1.0));
        });
    });
}

fn bench_polygon_creation(c: &mut Criterion) {
    c.bench_function("polygon_regular_hexagon", |b| {
        b.iter(|| {
            let _polygon = Polygon::regular(black_box(6), black_box(1.0));
        });
    });
}

fn bench_mobject_group(c: &mut Criterion) {
    c.bench_function("mobject_group_add_10", |b| {
        b.iter(|| {
            let mut group = MobjectGroup::new();
            for _ in 0..10 {
                group.add(Box::new(Circle::new(1.0)));
            }
            black_box(group);
        });
    });
}

fn bench_mobject_clone(c: &mut Criterion) {
    let circle = Circle::builder()
        .radius(2.0)
        .stroke_color(Color::BLUE)
        .fill_color(Color::RED)
        .build();

    c.bench_function("mobject_clone", |b| {
        b.iter(|| {
            let _cloned = black_box(&circle).clone_mobject();
        });
    });
}

fn bench_transform_application(c: &mut Criterion) {
    use manim_rs::core::Transform;

    let transform = Transform::translate(1.0, 2.0);

    c.bench_function("transform_circle", |b| {
        b.iter(|| {
            let mut circle = Circle::new(1.0);
            circle.apply_transform(black_box(&transform));
            black_box(circle);
        });
    });
}

criterion_group!(
    mobject_benches,
    bench_circle_creation,
    bench_circle_builder,
    bench_rectangle_creation,
    bench_polygon_creation,
    bench_mobject_group,
    bench_mobject_clone,
    bench_transform_application,
);

criterion_main!(mobject_benches);
