use block_chunk::Chunk;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn chunk_benchmark(c: &mut Criterion) {
    {
        let mut chunk = Chunk::<u32, 64>::default();
        c.bench_function("get/set 64", |b| {
            b.iter(|| {
                chunk.set(10, &(0, 0, 0).into());
                black_box(chunk.get(&(0, 0, 0).into()));
            })
        });
    }
    {
        let mut chunk = Chunk::<u32, 32>::default();
        c.bench_function("get/set 32", |b| {
            b.iter(|| {
                chunk.set(10, &(0, 0, 0).into());
                black_box(chunk.get(&(0, 0, 0).into()));
            })
        });
    }
    {
        let mut chunk = Chunk::<u32, 16>::default();
        c.bench_function("get/set 16", |b| {
            b.iter(|| {
                chunk.set(10, &(0, 0, 0).into());
                black_box(chunk.get(&(0, 0, 0).into()));
            })
        });
    }
    {
        let mut chunk = Chunk::<u32, 8>::default();
        c.bench_function("get/set 8", |b| {
            b.iter(|| {
                chunk.set(10, &(0, 0, 0).into());
                black_box(chunk.get(&(0, 0, 0).into()));
            })
        });
    }
    {
        let mut chunk = Chunk::<u32, 4>::default();
        c.bench_function("get/set 4", |b| {
            b.iter(|| {
                chunk.set(10, &(0, 0, 0).into());
                black_box(chunk.get(&(0, 0, 0).into()));
            })
        });
    }

    // COMPRESS

    c.bench_function("compress 64", |b| {
        b.iter(|| black_box(Chunk::<u32, 64>::default().compress()))
    });

    c.bench_function("compress 32", |b| {
        b.iter(|| black_box(Chunk::<u32, 32>::default().compress()))
    });

    c.bench_function("compress 16", |b| {
        b.iter(|| black_box(Chunk::<u32, 16>::default().compress()))
    });

    c.bench_function("compress 8", |b| {
        b.iter(|| black_box(Chunk::<u32, 8>::default().compress()))
    });

    c.bench_function("compress 4", |b| {
        b.iter(|| black_box(Chunk::<u32, 4>::default().compress()))
    });
}

criterion_group!(benches, chunk_benchmark);
criterion_main!(benches);
