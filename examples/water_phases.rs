// https://www.uml-diagrams.org/examples/water-phase-uml-state-machine-diagram-example.html

use std::any::{Any, TypeId};
use std::fmt::Debug;
use umlsm::guard::GuardedTransition;
use umlsm::state::{Cast, SimpleVertex};
use umlsm::transition::{ftrans, Transition};
use umlsm::StateMachine;

// States
#[derive(Debug, Clone)]
struct LiquidWater;
#[derive(Debug, Clone)]
struct WaterVapor;
#[derive(Debug, Clone)]
struct Plasma;
#[derive(Debug, Clone)]
struct IceOrFrost;

// Events
#[derive(Debug, PartialEq)]
enum Event {
    Ionize,
    Deionize,
    Vaporize,
    Condensate,
    Melt,
    Freeze,
    Deposition,
    Sublimation,
}

trait MyState: Debug + Any {
    fn tid(&self) -> TypeId;
}
impl<T: Debug + Any + 'static> MyState for T {
    fn tid(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl<T: Debug + Any> Cast<T> for dyn MyState {
    fn upcast(from: Box<T>) -> Box<Self> {
        from
    }

    fn upcast_ref(from: &T) -> &Self {
        from
    }

    fn concrete_tid(&self) -> TypeId {
        self.tid()
    }
}

type Sm = StateMachine<Event, (), dyn MyState>;

fn create_sm() -> Sm {
    Sm::with_default_state(SimpleVertex::with_data(LiquidWater).boxed())
        .register_vertex(SimpleVertex::with_data(WaterVapor).boxed())
        .register_vertex(SimpleVertex::with_data(Plasma).boxed())
        .register_vertex(SimpleVertex::with_data(IceOrFrost).boxed())
        .transition(switch_state(WaterVapor, Event::Ionize, Plasma))
        .transition(switch_state(Plasma, Event::Deionize, WaterVapor))
        .transition(switch_state(LiquidWater, Event::Vaporize, WaterVapor))
        .transition(switch_state(WaterVapor, Event::Condensate, LiquidWater))
        .transition(switch_state(IceOrFrost, Event::Melt, LiquidWater))
        .transition(switch_state(LiquidWater, Event::Freeze, IceOrFrost))
        .transition(switch_state(WaterVapor, Event::Deposition, IceOrFrost))
        .transition(switch_state(IceOrFrost, Event::Sublimation, WaterVapor))
}

fn switch_state<P: Debug + 'static, N: Debug + Clone + 'static>(
    _from: P,
    event: Event,
    to: N,
) -> impl Transition<Event, dyn MyState, Answer = ()> {
    GuardedTransition::new()
        .guard(move |e: &Event| *e == event)
        .transition(ftrans(move |_prev: P, _event| (to.clone(), ())))
}

fn main() {
    let sm = create_sm();
    repl(sm)
}

fn repl(mut sm: Sm) -> ! {
    use std::io::Write;

    loop {
        println!("|| Current state is {:?}", sm.current_state());
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).unwrap();

        let str = cmd.trim();
        let event = match str {
            "ionize" => Event::Ionize,
            "deionize" => Event::Deionize,
            "vaporize" => Event::Vaporize,
            "condensate" => Event::Condensate,
            "melt" => Event::Melt,
            "freeze" => Event::Freeze,
            "deposition" => Event::Deposition,
            "sublimation" => Event::Sublimation,
            _ => {
                println!("Unknown event.");
                continue;
            }
        };

        match sm.process(event) {
            Ok(_) => {
                println!("Success transition!");
            }
            Err(_) => {
                println!("No transition witch such event from such vertex.");
            }
        };
    }
}
