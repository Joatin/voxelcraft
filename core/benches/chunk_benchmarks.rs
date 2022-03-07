use criterion::{black_box, criterion_group, criterion_main, Criterion};
use voxelcraft_core::chunk::ChunkPosition;

pub fn chunk_position_benchmark(c: &mut Criterion) {
    let position = ChunkPosition::default();
    c.bench_function("surrounding_chunk 16", |b| {
        b.iter(|| position.surrounding_chunks(black_box(16)))
    });
    c.bench_function("surrounding_chunk 8", |b| {
        b.iter(|| position.surrounding_chunks(black_box(8)))
    });
    c.bench_function("surrounding_chunk 4", |b| {
        b.iter(|| position.surrounding_chunks(black_box(4)))
    });
}

criterion_group!(benches, chunk_position_benchmark);
criterion_main!(benches);
