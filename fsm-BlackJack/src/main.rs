use rust_fsm::*;

#[derive(Debug)]
enum BlackJackInput { // these are the possible transitions
    Deal,
    Hit,
    Stand,
    Bust,
    Draw,
    Over17,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BlackJackState { // these are the possible game states
    Start,
    PlayerTurn,
    DealerTurn,
    Results,
    DealerBust,
}

#[derive(Debug)]
struct BlackJackMachine;

#[derive(Debug, PartialEq)]
enum AccessDenied;

let mut Permission = True; // set true as stand in
let mut Score = 0; // stand in

impl StateMachineImpl for BlackJackMachine {
    type Input = BlackJackInput;
    type State = BlackJackState;
    type Output = AccessDenied;
    const INITIAL_STATE: Self::State = BlackJackState::Closed;

    fn transition(state: &Self::State, input: &Self::Input) -> Option<Self::State> {
        match (state, input) {
            (BlackJackState::Start, BlackJackInput::Deal) => {
                Some(BlackJackState::PlayerTurn)
            }
            (BlackJackState::PlayerTurn, BlackJackInput::Hit) => {
                Some(BlackJackState::PlayerTurn)
            }
            (BlackJackState::PlayerTurn, BlackJackInput::Stand) => {
                Some(BlackJackState::DealerTurn)
            }
            (BlackJackState::PlayerTurn, BlackJackInput::Bust) => {
                Some(BlackJackState::Results)
            }
            (BlackJackState::DealerTurn, BlackJackInput::Draw) => {
                Some(BlackJackState::DealerTurn)
            }
            (BlackJackState::DealerTurn, BlackJackInput::Over17) => {
                Some(BlackJackState::DealerBust)
            }
            _ => None,
        }
    }

    fn output(state: &Self::State, input: &Self::Input) -> Option<Self::Output> {
        match (state, input) {
            (BlackJackState::PlayerTurn, BlackJackInput::Hit) => {
                if (Permission != True){
                    Some(AccessDenied)
                }
            }
            (BlackJackState::PlayerTurn, BlackJackInput::Stand) => {
                if (Permission != True){
                    Some(AccessDenied)
                }
            }
            _ => Score,
        }
    }

}


