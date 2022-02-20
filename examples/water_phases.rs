// https://www.uml-diagrams.org/examples/water-phase-uml-state-machine-diagram-example.html

use std::any::{Any, TypeId};
use std::fmt::Debug;
use umlsm::state::{Cast, SimpleVertex};
use umlsm::transition::{ftrans, Transition};
use umlsm::{Event, StateMachine};

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
#[derive(Debug)]
struct Ionize;
#[derive(Debug)]
struct Deionize;
#[derive(Debug)]
struct Vaporize;
#[derive(Debug)]
struct Condensate;
#[derive(Debug)]
struct Melt;
#[derive(Debug)]
struct Freeze;
#[derive(Debug)]
struct Deposition;
#[derive(Debug)]
struct Sublimation;

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

type Sm = StateMachine<(), dyn MyState>;

fn create_sm() -> Sm {
    Sm::with_default_state(SimpleVertex::with_data(LiquidWater).boxed())
        .register_vertex(SimpleVertex::with_data(WaterVapor).boxed())
        .register_vertex(SimpleVertex::with_data(Plasma).boxed())
        .register_vertex(SimpleVertex::with_data(IceOrFrost).boxed())
        .transition(switch_state(WaterVapor, Ionize, Plasma))
        .transition(switch_state(Plasma, Deionize, WaterVapor))
        .transition(switch_state(LiquidWater, Vaporize, WaterVapor))
        .transition(switch_state(WaterVapor, Condensate, LiquidWater))
        .transition(switch_state(IceOrFrost, Melt, LiquidWater))
        .transition(switch_state(LiquidWater, Freeze, IceOrFrost))
        .transition(switch_state(WaterVapor, Deposition, IceOrFrost))
        .transition(switch_state(IceOrFrost, Sublimation, WaterVapor))
}

fn switch_state<P, E, N>(_from: P, _event: E, to: N) -> impl Transition<dyn MyState, Answer = ()>
where
    P: Debug + 'static,
    E: 'static,
    N: Debug + Clone + 'static,
{
    ftrans(move |_prev: P, _event: E| (to.clone(), ()))
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
            "ionize" => Box::new(Ionize) as Event,
            "deionize" => Box::new(Deionize),
            "vaporize" => Box::new(Vaporize),
            "condensate" => Box::new(Condensate),
            "melt" => Box::new(Melt),
            "freeze" => Box::new(Freeze),
            "deposition" => Box::new(Deposition),
            "sublimation" => Box::new(Sublimation),
            _ => {
                println!("Unknown event.");
                continue;
            }
        };

        match sm.process_boxed(event) {
            Ok(_) => {
                println!("Success transition!");
            }
            Err(_) => {
                println!("No transition witch such event from such vertex.");
            }
        };
    }
}
