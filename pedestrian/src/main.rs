use std::{thread, time};
use std::collections::HashMap;

#[derive(PartialEq)]
enum TrafficLight{
    Red,
    Yellow,
    Green,
}

#[derive(PartialEq)]
enum CrossWalkSign{
    Walk,
    Countdown,
    Stop,
}

struct Intersection{
    light: TrafficLight,
    crosswalk: CrossWalkSign,
}

enum IntersectionState{
    CarThrough,
    PedThrough,
    Stopped,
}

fn changeState(intersection: &mut Intersection) {
    intersection.light = TrafficLight::Yellow;
    println!("Light is yellow, crosswalk is stopped");
    thread::sleep(time::Duration::from_secs(5));
    
    intersection.light = TrafficLight::Red;
    println!("Light is red, crosswalk is stopped");
    thread::sleep(time::Duration::from_secs(2));
    
    intersection.crosswalk = CrossWalkSign::Walk; 
    println!("Light is red, crosswalk is on");
    thread::sleep(time::Duration::from_secs(10));
    intersection.crosswalk = CrossWalkSign::Countdown; 
    println!("Light is red, crosswalk is counting down");
    thread::sleep(time::Duration::from_secs(10));    
    intersection.crosswalk = CrossWalkSign::Stop;
    println!("Light is red, crosswalk is stopped");
    thread::sleep(time::Duration::from_secs(2));

    intersection.light = TrafficLight::Green;
    println!("Light is green, crosswalk is stopped. Crossing is not available for 20 seconds");
    thread::sleep(time::Duration::from_secs(20));
}


fn main() {
    let mut intersection = Intersection{
        light: TrafficLight::Green,
        crosswalk: CrossWalkSign::Stop,
    };
    println!("Traffic light is green, press a key to cross road");

    // User input to cross
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    
    if b1 > 0 {
        // call change state function
        changeState(&mut intersection);
    }

}