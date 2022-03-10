use block_chunk::mesh::fast_mesh;
use block_chunk::mesh::greedy_mesh;
use block_chunk::mesh::BlockDescriptor;
use block_chunk::Chunk;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_set<const SIZE: usize>(c: &mut Criterion) {
    {
        let chunk = Chunk::<u32, SIZE>::default();
        c.bench_function(&format!("chunk_{} fast_mesh box", SIZE), |b| {
            b.iter(|| {
                fast_mesh(&chunk, |_| {
                    Some(BlockDescriptor {
                        is_standard_square: true,
                        is_transparent: false,
                        texture_id: (),
                    })
                })
            })
        });
    }

    {
        let chunk = Chunk::<u32, SIZE>::new_checker(0, 1);
        c.bench_function(&format!("chunk_{} fast_mesh checker", SIZE), |b| {
            b.iter(|| {
                fast_mesh(&chunk, |val| {
                    if *val == 0 {
                        None
                    } else {
                        Some(BlockDescriptor {
                            is_standard_square: true,
                            is_transparent: false,
                            texture_id: (),
                        })
                    }
                })
            })
        });
    }

    {
        let chunk = Chunk::<u32, SIZE>::default();
        c.bench_function(&format!("chunk_{} greedy_mesh box", SIZE), |b| {
            b.iter(|| {
                greedy_mesh(&chunk, |_| {
                    Some(BlockDescriptor {
                        is_standard_square: true,
                        is_transparent: false,
                        texture_id: (),
                    })
                })
            })
        });
    }

    {
        let chunk = Chunk::<u32, SIZE>::new_checker(0, 1);
        c.bench_function(&format!("chunk_{} greedy_mesh checker", SIZE), |b| {
            b.iter(|| {
                greedy_mesh(&chunk, |val| {
                    if *val == 0 {
                        None
                    } else {
                        Some(BlockDescriptor {
                            is_standard_square: true,
                            is_transparent: false,
                            texture_id: (),
                        })
                    }
                })
            })
        });
    }
}

pub fn meshable_chunk(c: &mut Criterion) {
    benchmark_set::<64>(c);
    benchmark_set::<32>(c);
    benchmark_set::<16>(c);
    benchmark_set::<8>(c);
    benchmark_set::<4>(c);
}

criterion_group!(benches, meshable_chunk);
criterion_main!(benches);
