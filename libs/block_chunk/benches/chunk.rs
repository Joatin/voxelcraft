use block_chunk::Chunk;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn benchmark_set<const SIZE: usize>(c: &mut Criterion) {
    {
        let mut chunk = Chunk::<u32, SIZE>::default();
        c.bench_function(&format!("chunk_{} get/set", SIZE), |b| {
            b.iter(|| {
                chunk.set(10, &(0, 0, 0).into());
                black_box(chunk.get(&(0, 0, 0).into()));
            })
        });
    }

    {
        let chunk = Chunk::<u32, SIZE>::default();
        c.bench_function(&format!("chunk_{} compress", SIZE), |b| {
            b.iter_batched(
                || chunk.clone(),
                |chunk| chunk.compress(),
                BatchSize::SmallInput,
            )
        });
    }
}

pub fn chunk_benchmark(c: &mut Criterion) {
    benchmark_set::<64>(c);
    benchmark_set::<32>(c);
    benchmark_set::<16>(c);
    benchmark_set::<8>(c);
    benchmark_set::<4>(c);
}

criterion_group!(benches, chunk_benchmark);
criterion_main!(benches);
