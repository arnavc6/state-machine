// current_state = transitions{to: current_state, event: event_name}
use std::collections::HashMap;

struct StateMachine{
    current_state: String,
    transitions: HashMap<(String, String), String>
}

impl StateMachine{
    pub fn new (start_state: &str, table: Vec<Vec<&str>>) -> Self{
        let mut transitions = HashMap::new();
        for row in table.into_iter(){
            transitions.insert((row[0].to_string(), row[1].to_string()), row[2].to_string());
        }
        let current_state = start_state.to_string();
        Self { current_state, transitions }
    }
    pub fn get_state(&self) -> String{
        (*self.current_state).to_string()
    }
    pub fn change_state(&mut self, transition: &str) -> bool{
        let str_transition = transition.to_string();
        match self.transitions.get(&((*(self.current_state)).to_string(), str_transition)){
            Some(new_state) => {
                self.current_state = new_state.as_str().to_string();
                true
            }
            _ => false
        }
    }
}

fn main() {
    let transition_table = vec![
        vec!["locked", "turnKey", "unlocked"],
        vec!["unlocked", "turnKey", "locked"],
        vec!["locked", "break", "broken"],
        vec!["unlocked", "break", "broken"],
        vec!["broken", "fix", "locked"]
    ];
    let first_state = "locked";
    let mut state_machine = StateMachine::new(first_state, transition_table.clone());
    println!("Initial state {}", state_machine.get_state());
    for transition in transition_table.into_iter(){
        match state_machine.change_state(transition[1]){
            true => println!("Transitioned to {} using {}", state_machine.get_state(), transition[1]),
            false => println!("Transition {} does not exist for state {}", transition[1], state_machine.get_state())
        }
    }
}
