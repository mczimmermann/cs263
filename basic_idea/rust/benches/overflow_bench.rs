use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_overflow::{calculate_buffer_size_checked, calculate_buffer_size_wrapping};

fn benchmark_checked(c: &mut Criterion) {
    c.bench_function("rust_checked", |b| {
        b.iter(|| {
            let size = calculate_buffer_size_checked(
                black_box(1024),
                black_box(768)
            );
            black_box(size);
        });
    });
}

fn benchmark_checked_overflow(c: &mut Criterion) {
    c.bench_function("rust_checked_overflow", |b| {
        b.iter(|| {
            let size = calculate_buffer_size_checked(
                black_box(u32::MAX / 2),  // Will overflow
                black_box(3)
            );
            black_box(size);  // Returns None
        });
    });
}

// fn benchmark_wrapping(c: &mut Criterion) {
//     c.bench_function("rust_wrapping", |b| {
//         b.iter(|| {
//             let size = calculate_buffer_size_wrapping(
//                 black_box(1024),
//                 black_box(768)
//             );
//             black_box(size);
//         });
//     });
// }

criterion_group!(benches, benchmark_checked, benchmark_checked_overflow);
criterion_main!(benches);