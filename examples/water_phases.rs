// https://www.uml-diagrams.org/examples/water-phase-uml-state-machine-diagram-example.html

use std::any::{Any, TypeId};
use std::fmt::Debug;
use umlsm::state::{Cast, InitialPseudoState, SimpleVertex};
use umlsm::transition::{ftrans, Transition};
use umlsm::{events, states, EnterSmEvent, Event, SmBuilder};

events! {
    #[derive(Debug)]
    {
        struct Ionize;
        struct Deionize;
        struct Vaporize;
        struct Condensate;
        struct Melt;
        struct Freeze;
        struct Deposition;
        struct Sublimation;
    }
}

states! {
    trait MyState: Debug;

    #[derive(Debug, Clone)]
    {
        struct LiquidWater;
        struct WaterVapor;
        struct Plasma;
        struct IceOrFrost;
    }
}

type Sm = umlsm::Sm<dyn MyState>;

fn create_sm() -> Sm {
    SmBuilder::new()
        .register_vertex(SimpleVertex::with_data(LiquidWater).to_vertex())
        .register_vertex(SimpleVertex::with_data(WaterVapor).to_vertex())
        .register_vertex(SimpleVertex::with_data(Plasma).to_vertex())
        .register_vertex(SimpleVertex::with_data(IceOrFrost).to_vertex())
        .transition(switch_state(InitialPseudoState, EnterSmEvent, LiquidWater))
        .transition(switch_state(WaterVapor, Ionize, Plasma))
        .transition(switch_state(Plasma, Deionize, WaterVapor))
        .transition(switch_state(LiquidWater, Vaporize, WaterVapor))
        .transition(switch_state(WaterVapor, Condensate, LiquidWater))
        .transition(switch_state(IceOrFrost, Melt, LiquidWater))
        .transition(switch_state(LiquidWater, Freeze, IceOrFrost))
        .transition(switch_state(WaterVapor, Deposition, IceOrFrost))
        .transition(switch_state(IceOrFrost, Sublimation, WaterVapor))
        .build()
        .unwrap()
}

fn switch_state<P, E, N>(_from: P, _event: E, to: N) -> impl Transition<dyn MyState>
where
    P: Debug + 'static,
    E: 'static,
    N: Debug + Clone + 'static,
{
    ftrans(move |_prev: P, _event: E| to.clone())
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
