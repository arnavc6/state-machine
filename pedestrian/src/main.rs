use std::{thread, time};
use std::collections::HashMap;

#[derive(PartialEq)]
enum TrafficLight{
    Green,
    Yellow,
    Red
}

#[derive(PartialEq)]
enum CrossWalkSign{
    Ready,
    Waiting(u64),
    Walk(u64),
    Countdown(u64),
    Blocked(u64)
}

static WAIT_TIME: u64 = 8;
static CROSS_TIME: u64 = 20;
static BLOCK_TIME: u64 = 20;

/**
 * 1. Green/Ready
 * 2. Yellow/Waiting
 * 3. Red/Waiting
 * 4. Red/Walk
 * 5. Red/Countdown
 * 6. Red/Blocked
 * 7. Green/Blocked
 * loop back
 */

struct Intersection{
    light: TrafficLight,
    crosswalk: CrossWalkSign,
}

enum IntersectionState{
    CarThrough,
    PedThrough,
    Stopped,
}

fn change_state(intersection: &mut Intersection) {
    intersection.crosswalk = CrossWalkSign::Waiting(WAIT_TIME);
    match intersection.crosswalk{
        CrossWalkSign::Waiting(x) => {
            intersection.light = TrafficLight::Yellow;
            println!("Light is yellow, {} seconds until crossing", x);
            thread::sleep(time::Duration::from_secs(x/2));
            intersection.light = TrafficLight::Red;
            println!("Light is red, {} seconds until crossing", x/2);
            thread::sleep(time::Duration::from_secs(x/2));
        }
        _ => {
            println!("crosswalk machine 🅱️roke")
        }
    }

    intersection.crosswalk = CrossWalkSign::Walk(CROSS_TIME);
    match intersection.crosswalk{
        CrossWalkSign::Walk(x) => {
            println!("Walk sign is on to cross, {} seconds remaining", x);
            thread::sleep(time::Duration::from_secs(x/2));
        }
        _ => {
            println!("crosswalk machine 🅱️roke")
        }
    }

    intersection.crosswalk = CrossWalkSign::Countdown(CROSS_TIME/2);
    match intersection.crosswalk{
        CrossWalkSign::Countdown(x) => {
            let mut y = x;
            while y > 0{
                println!("Counting down... {} seconds remaining", y);
                thread::sleep(time::Duration::from_secs(1));
                y -= 1;
            }
        }
        _ => {
            println!("crosswalk machine 🅱️roke")
        }
    }
    
    intersection.crosswalk = CrossWalkSign::Blocked(BLOCK_TIME);
    match intersection.crosswalk{
        CrossWalkSign::Blocked(x) => {
            println!("Crossing will be available in {} seconds", x);
            thread::sleep(time::Duration::from_secs(x));
        }
        _ => {
            println!("crosswalk machine 🅱️roke")
        }
    }
}


fn main() {
    let mut intersection = Intersection{
        light: TrafficLight::Green,
        crosswalk: CrossWalkSign::Ready,
    };
    
    while true{
        println!("Traffic light is green, press a key to cross road or hit enter to exit");

        // User input to cross
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        
        if b1 > 1 {
            // call change state function
            change_state(&mut intersection);
        } else{
            break;
        }
    }
}