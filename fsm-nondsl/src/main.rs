use rust_fsm::*;

#[derive(Debug)]
enum DoorInput {
    Key,
    Break,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DoorState {
    Closed,
    Open,
    Broken,
}

#[derive(Debug)]
struct DoorMachine;

#[derive(Debug, PartialEq)]
struct DoorOutputSetTimer;

impl StateMachineImpl for DoorMachine {
    type Input = DoorInput;
    type State = DoorState;
    type Output = DoorOutputSetTimer;
    const INITIAL_STATE: Self::State = DoorState::Closed;

    fn transition(state: &Self::State, input: &Self::Input) -> Option<Self::State> {
        match (state, input) {
            (DoorState::Closed, DoorInput::Key) => {
                Some(DoorState::Open)
            }
            (DoorState::Open, DoorInput::Key) => {
                Some(DoorState::Closed)
            }
            (DoorState::Closed, DoorInput::Break) => {
                Some(DoorState::Broken)
            }
            (DoorState::Open, DoorInput::Break) => {
                Some(DoorState::Broken)
            }
            _ => None,
        }
    }

    fn output(state: &Self::State, input: &Self::Input) -> Option<Self::Output> {
        match (state, input) {
            _ => None,
        }
    }

}

fn main() {
    let mut machine: StateMachine<DoorMachine> = StateMachine::new();
    machine.consume(&DoorInput::Key).unwrap();
    println!("{:?}", machine.state());
    
    machine.consume(&DoorInput::Key).unwrap();
    println!("{:?}", machine.state());
    
    machine.consume(&DoorInput::Break).unwrap();
    println!("{:?}", machine.state());
}

