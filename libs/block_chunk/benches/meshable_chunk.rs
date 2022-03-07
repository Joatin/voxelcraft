use criterion::{black_box, criterion_group, criterion_main, Criterion};
use block_chunk::Chunk;
use block_chunk::mesh::{MeshableChunk, BlockDescriptor};
use tokio::runtime::Runtime;

pub fn meshable_chunk(c: &mut Criterion) {
    // FAST MESH BOX
    {
        let chunk = Chunk::<u32, 64>::default();
        c.bench_function("fast_mesh box 64", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 32>::default();
        c.bench_function("fast_mesh box 32", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 16>::default();
        c.bench_function("fast_mesh box 16", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 8>::default();
        c.bench_function("fast_mesh box 8", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 4>::default();
        c.bench_function("fast_mesh box 4", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }

    // FAST MESH CHECKER
    {
        let chunk = Chunk::<u32, 64>::new_checker(0, 1);
        c.bench_function("fast_mesh checker 64", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 32>::default();
        c.bench_function("fast_mesh checker 32", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 16>::default();
        c.bench_function("fast_mesh checker 16", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 8>::default();
        c.bench_function("fast_mesh checker 8", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 4>::default();
        c.bench_function("fast_mesh checker 4", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.fast_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }

    // GREEDY MESH

    {
        let chunk = Chunk::<u32, 64>::default();
        c.bench_function("greedy_mesh box 64", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 32>::default();
        c.bench_function("greedy_mesh box 32", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 16>::default();
        c.bench_function("greedy_mesh box 16", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 8>::default();
        c.bench_function("greedy_mesh box 8", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }
    {
        let chunk = Chunk::<u32, 4>::default();
        c.bench_function("greedy_mesh box 4", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|_| black_box(Some(BlockDescriptor { is_standard_square: true, is_transparent: false })))
            })
        });
    }

    // GREEDY MESH CHECKER
    {
        let chunk = Chunk::<u32, 64>::new_checker(0, 1);
        c.bench_function("greedy_mesh checker 64", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 32>::default();
        c.bench_function("greedy_mesh checker 32", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 16>::default();
        c.bench_function("greedy_mesh checker 16", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 8>::default();
        c.bench_function("greedy_mesh checker 8", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
    {
        let chunk = Chunk::<u32, 4>::default();
        c.bench_function("greedy_mesh checker 4", |b| {
            let runtime = Runtime::new().unwrap();
            b.to_async(runtime).iter(|| {
                chunk.greedy_mesh(|val| if *val == 0 {
                    None
                } else {
                    Some(BlockDescriptor { is_standard_square: true, is_transparent: false })
                })
            })
        });
    }
}

criterion_group!(benches, meshable_chunk);
criterion_main!(benches);