use std::{thread, time};
use std::collections::HashMap;

enum Light{
    Green,
    Yellow,
    Red,
}

fn create_intersection() -> HashMap<&'static str, Light>{
    let mut isec = HashMap::new();
    isec.insert("north-rs", Light::Red);
    isec.insert("north-l", Light::Red);
    isec.insert("south-rs", Light::Red);
    isec.insert("south-l", Light::Red);
    isec.insert("east-rs", Light::Red);
    isec.insert("east-l", Light::Red);
    isec.insert("west-rs", Light::Red);
    isec.insert("west-l", Light::Red);
    isec
}
fn change_one(isect: &mut HashMap<&'static str, Light>, dir: &'static str){
    isect.insert(dir, Light::Green);
    println!("direction {} is green", dir);
    thread::sleep(time::Duration::from_secs(2));
    isect.insert(dir, Light::Yellow);
    println!("direction {} is yellow", dir);
    thread::sleep(time::Duration::from_secs(2));
    isect.insert(dir, Light::Red);
    println!("direction {} is red", dir);
    thread::sleep(time::Duration::from_secs(2));
}
fn change_two(isect: &mut HashMap<&'static str, Light>, dir1: &'static str, dir2: &'static str){
    isect.insert(dir1, Light::Green);
    isect.insert(dir2, Light::Green);
    println!("directions {} and {} are green", dir1, dir2);
    thread::sleep(time::Duration::from_secs(2));
    isect.insert(dir1, Light::Yellow);
    isect.insert(dir2, Light::Yellow);
    println!("directions {} and {} are yellow", dir1, dir2);
    thread::sleep(time::Duration::from_secs(2));
    isect.insert(dir1, Light::Red);
    isect.insert(dir2, Light::Red);
    println!("directions {} and {} are red", dir1, dir2);
    thread::sleep(time::Duration::from_secs(2));
}
fn single_rotation(isect: &mut HashMap<&'static str, Light>){ // one at a time
    change_one(isect, "north-rs");
    change_one(isect, "north-l");
    change_one(isect, "south-rs");
    change_one(isect, "south-l");
    change_one(isect, "east-rs");
    change_one(isect, "east-l");
    change_one(isect, "west-rs");
    change_one(isect, "west-l");
}
fn directional_rotation(isect: &mut HashMap<&'static str, Light>){ // NESW
    change_two(isect, "north-rs", "north-l");
    change_two(isect, "south-rs", "south-l");
    change_two(isect, "east-rs", "east-l");
    change_two(isect, "west-rs", "west-l");
}
fn opposite_rotation(isect: &mut HashMap<&'static str, Light>){ // opposite direction right and straight, then opposite direction left
    change_two(isect, "north-rs", "south-rs");
    change_two(isect, "north-l", "south-l");
    change_two(isect, "east-rs", "west-rs");
    change_two(isect, "east-l", "west-l");
}
fn main() {
    let mut isect = create_intersection();
    single_rotation(&mut isect);
    directional_rotation(&mut isect);
    opposite_rotation(&mut isect);
}
