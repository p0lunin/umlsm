// // https://www.uml-diagrams.org/examples/water-phase-uml-state-machine-diagram-example.html
//
// use std::any::{Any, TypeId};
// use std::fmt::Debug;
// use umlsm::state::{Cast, InitialPseudoState, SimpleVertex};
// use umlsm::transition::{ftrans, Switch, Transition};
// use umlsm::{events, states, switch, EnterSmEvent, Event, SmBuilder};
//
// events! {
//     #[derive(Debug)]
//     {
//         struct Ionize;
//         struct Deionize;
//         struct Vaporize;
//         struct Condensate;
//         struct Melt;
//         struct Freeze;
//         struct Deposition;
//         struct Sublimation;
//     }
// }
//
// states! {
//     trait MyState: Debug;
//
//     #[derive(Debug, Clone)]
//     {
//         struct LiquidWater;
//         struct WaterVapor;
//         struct Plasma;
//         struct IceOrFrost;
//     }
// }
//
// type Sm = umlsm::Sm<dyn MyState>;
//
// fn create_sm() -> Sm {
//     SmBuilder::new()
//         .register_vertex(SimpleVertex::with_data(LiquidWater).to_vertex())
//         .register_vertex(SimpleVertex::with_data(WaterVapor).to_vertex())
//         .register_vertex(SimpleVertex::with_data(Plasma).to_vertex())
//         .register_vertex(SimpleVertex::with_data(IceOrFrost).to_vertex())
//         .transition(switch!(InitialPseudoState + EnterSmEvent = LiquidWater))
//         .transition(switch!(WaterVapor + Ionize = Plasma))
//         .transition(switch!(Plasma + Deionize = WaterVapor))
//         .transition(switch!(LiquidWater + Vaporize = WaterVapor))
//         .transition(switch!(WaterVapor + Condensate = LiquidWater))
//         .transition(switch!(IceOrFrost + Melt = LiquidWater))
//         .transition(switch!(LiquidWater + Freeze = IceOrFrost))
//         .transition(switch!(WaterVapor + Deposition = IceOrFrost))
//         .transition(switch!(IceOrFrost + Sublimation = WaterVapor))
//         .build()
//         .unwrap()
// }
//
// fn main() {
//     let sm = create_sm();
//     repl(sm)
// }
//
// fn repl(mut sm: Sm) -> ! {
//     use std::io::Write;
//
//     loop {
//         println!("|| Current state is {:?}", sm.current_state());
//         print!(">> ");
//         std::io::stdout().flush().unwrap();
//
//         let mut cmd = String::new();
//         std::io::stdin().read_line(&mut cmd).unwrap();
//
//         let str = cmd.trim();
//         let event = match str {
//             "ionize" => Box::new(Ionize) as Event,
//             "deionize" => Box::new(Deionize),
//             "vaporize" => Box::new(Vaporize),
//             "condensate" => Box::new(Condensate),
//             "melt" => Box::new(Melt),
//             "freeze" => Box::new(Freeze),
//             "deposition" => Box::new(Deposition),
//             "sublimation" => Box::new(Sublimation),
//             _ => {
//                 println!("Unknown event.");
//                 continue;
//             }
//         };
//
//         match sm.process_boxed(event) {
//             Ok(_) => {
//                 println!("Success transition!");
//             }
//             Err(_) => {
//                 println!("No transition witch such event from such vertex.");
//             }
//         };
//     }
// }
fn main() {}