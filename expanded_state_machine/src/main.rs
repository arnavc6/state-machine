use std::{thread, time};
use std::collections::HashMap;

#[derive(PartialEq)]
enum Direction{
    Still,
    Forward,
    Backward,
    FrontRightSwing,
    FrontLeftSwing,
    BackRightSwing,
    BackLeftSwing,
    RightPoint,
    LeftPoint, 
}

struct Robot{
    rpm_l: i32,
    rpm_r: i32,
}

fn dir(rob: &mut Robot) -> Direction{
    let ll = rob.rpm_l;
    let rl = rob.rpm_r;
    if ll > 0 {
        if rl < 0{
            Direction::LeftPoint
        } 
        else if rl >= 0{
            if rl < ll{
                Direction::FrontLeftSwing
            } 
            else if rl > ll{
                Direction::FrontRightSwing
            } 
            else{
                Direction::Forward
            }
        }
        else{
            Direction::Forward
        }
    } else if ll < 0{
        if rl > 0{
            Direction::RightPoint
        } else if rl <= 0{
            if rl > ll{
                Direction::BackLeftSwing
            } else if rl < ll{
                Direction::BackRightSwing
            } else{
                Direction::Backward
            }
        }
        else{
            Direction::Backward
        }
    } else{
        if rl < 0{
            Direction::BackRightSwing
        } else if rl > 0{
            Direction::FrontRightSwing
        } else{
            Direction::Still
        }
    }
}
fn print_dir(dir: Direction){
    if dir == Direction::Still{
        println!("robot is still");
    } else if dir == Direction::Forward{
        println!("robot is moving forward");
    } else if dir == Direction::Backward{
        println!("robot is moving backward");
    } else if dir == Direction::FrontRightSwing{
        println!("robot is swinging right");
    } else if dir == Direction::FrontLeftSwing{
        println!("robot is swinging left");
    } else if dir == Direction::BackRightSwing{
        println!("robot is backswinging right");
    } else if dir == Direction::BackLeftSwing{
        println!("robot is backswinging left");
    } else if dir == Direction::RightPoint{
        println!("robot is turning right");
    } else if dir == Direction::LeftPoint{
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

    rob.rpm_r = -1;
    print_dir(dir(&mut rob));
    rob.rpm_l = -1;
    print_dir(dir(&mut rob));
    rob.rpm_r = 0;
    print_dir(dir(&mut rob));

    rob.rpm_r = 1;
    print_dir(dir(&mut rob));
    rob.rpm_l = 1;
    rob.rpm_r = -1;
    print_dir(dir(&mut rob));

    rob.rpm_l = 0;
    rob.rpm_r = 0;
    print_dir(dir(&mut rob));
}
