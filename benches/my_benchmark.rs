use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use umlsm::state::InitialPseudostate;
use umlsm::state::SimpleVertex;
use umlsm::transition::ftrans;
use umlsm::StateMachine;

struct MyState;

#[allow(unused_must_use)]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter_batched(
            || {
                StateMachine::new()
                    .register_vertex(SimpleVertex::<MyState>::new().boxed())
                    .transition(ftrans(|_: InitialPseudostate, event: i32| {
                        (MyState, event * 2)
                    }))
            },
            |mut s| {
                s.process(black_box(3));
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
