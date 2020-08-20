use criterion::{black_box, criterion_group, criterion_main};
use vm::prelude::*;

fn run<'a>(bytes: &'a [Byte]) {
    let mut memory = Memory::with_capacity(0x10000);
    memory.set_bytes(bytes);

    let mut cpu = Cpu::from(memory);
    cpu.run();
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("binary 1", |b| b.iter(|| {
        run(black_box(include_bytes!("../tests/binary1")))
    }));

    c.bench_function("binary 2", |b| b.iter(|| {
        run(black_box(include_bytes!("../tests/binary2")))
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
