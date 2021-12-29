use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use umlsm::{StateMachine};
use umlsm::state::InitialPseudostate;
use umlsm::transition::ftrans;
use umlsm::state::State;

struct MyState;

#[allow(unused_must_use)]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| {
        b.iter_batched(|| {
            StateMachine::new()
                .register_vertex(State::empty::<MyState>().boxed())
                .transition(ftrans(|_: InitialPseudostate, event: i32| {
                    (MyState, event * 2)
                }))
        }, |mut s| {
            s.process(black_box(3));
        }, BatchSize::SmallInput);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);