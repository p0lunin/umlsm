// use umlsm::{EnterSmEvent, events, SmBuilder, states, switch, Vertex};
// use umlsm::state::SimpleVertex;
// use umlsm::state::InitialPseudoState;
//
// events! {
//     #[derive(Debug)]
//     {
//         struct TurnOn;
//         struct TurnOff;
//         struct Service;
//         struct Failure;
//         struct InsertCard;
//         struct Cancel;
//
//         struct AuthentificationCompleted;
//         struct TransactionChosen;
//         struct TransactionCompleted;
//     }
// }
//
// states! {
//     trait MyState: std::fmt::Debug;
//
//     #[derive(Debug, Clone)]
//     {
//         struct Off;
//         struct SelfTest;
//         struct Idle;
//         struct Maintenance;
//         struct OutOfService;
//
//         struct ServingCustomer;
//         struct CustomerAuthentification;
//         struct SelectingTransaction;
//         struct Transaction;
//     }
// }
//
// type Sm = umlsm::Sm<dyn MyState>;
//
// fn create_sm() -> Sm {
//     SmBuilder::new()
//         .register_vertex(SimpleVertex::with_data(Off).to_vertex())
//         .register_vertex(SimpleVertex::with_data(SelfTest).to_vertex())
//         .register_vertex(SimpleVertex::with_data(Idle).to_vertex())
//         .register_vertex(SimpleVertex::with_data(Maintenance).to_vertex())
//         .register_vertex(SimpleVertex::with_data(OutOfService).to_vertex())
//         .register_vertex(Vertex::SubMachineState {
//             handler: Box::new(SimpleVertex::with_data(ServingCustomer)),
//             sm: SmBuilder::new()
//                 .register_vertex(SimpleVertex::with_data(CustomerAuthentification).to_vertex())
//                 .register_vertex(SimpleVertex::with_data(SelectingTransaction).to_vertex())
//                 .register_vertex(SimpleVertex::with_data(Transaction).to_vertex())
//                 .transition(switch!(InitialPseudoState + EnterSmEvent = CustomerAuthentification))
//                 .transition(switch!(CustomerAuthentification + AuthentificationCompleted = SelectingTransaction))
//                 .transition(switch!(SelectingTransaction + TransactionChosen = Transaction))
//                 .build()
//                 .unwrap()
//         })
//         .transition(switch!(InitialPseudoState + EnterSmEvent = Idle))
//         .transition(switch!(Idle + InsertCard = ServingCustomer))
//         .transition(switch!(ServingCustomer + TransactionCompleted = Idle))
//         .build()
//         .unwrap()
// }
//
// fn main() {
//     let mut sm = create_sm();
//     sm.process(InsertCard).unwrap();
//     dbg!(sm.current_state());
//     sm.process(AuthentificationCompleted).unwrap();
//     dbg!(sm.current_state());
//     sm.process(TransactionChosen).unwrap();
//     dbg!(sm.current_state());
//     sm.process(TransactionCompleted).unwrap();
//     dbg!(sm.current_state());
// }
fn main() {}