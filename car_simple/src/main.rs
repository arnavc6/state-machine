use std::{thread, time};
use std::collections::HashMap;

#[derive(PartialEq)]
enum Direction{
    Still,
    Forward,
    Backward,
    LeftTurn,
    RightTurn, 
}

struct Robot{
    rpm_l: i32,
    rpm_r: i32,
}

fn dir(rob: &mut Robot) -> Direction{
    let ll = rob.rpm_l;
    let rl = rob.rpm_r;
    
    if ll == 0 && rl == 0 {
        Direction::Still
    } else if rl > ll {
        Direction::LeftTurn
    } else if ll > rl {
        Direction::RightTurn
    } else if rl < 0 && ll < 0 {
        Direction::Backward
    } else {
        Direction::Forward
    }
}
fn print_dir(dir: Direction){
    if dir == Direction::Still{
        println!("robot is still");
    } else if dir == Direction::Forward{
        println!("robot is moving forward");
    } else if dir == Direction::Backward{
        println!("robot is moving backward");
    } else if dir == Direction::RightTurn{
        println!("robot is turning right");
    } else if dir == Direction::LeftTurn{
        println!("robot is turning left");
    }
}
fn main() {
    let mut rob = Robot{
        rpm_l: 0,
        rpm_r: 0,
    };
    print_dir(dir(&mut rob));

    rob.rpm_l = 1;
    print_dir(dir(&mut rob));
    rob.rpm_r = 1;
    print_dir(dir(&mut rob));
    rob.rpm_l = 0;
    print_dir(dir(&mut rob));
    println!("---");

    rob.rpm_r = -1;
    print_dir(dir(&mut rob));
    rob.rpm_l = -1;
    print_dir(dir(&mut rob));
    rob.rpm_r = 0;
    print_dir(dir(&mut rob));
    println!("---");

    rob.rpm_r = 1;
    print_dir(dir(&mut rob));
    rob.rpm_l = 1;
    rob.rpm_r = -1;
    print_dir(dir(&mut rob));
    println!("---");

    rob.rpm_l = 0;
    rob.rpm_r = 0;
    print_dir(dir(&mut rob));
}
