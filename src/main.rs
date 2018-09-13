mod cacher;
mod tests;

use cacher::Cacher;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);


    let fun = | x: u32 | {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        x
    };

    let mut c1 = Cacher::new(fun);

    println!("{}", c1.value(2));
    println!("{}", c1.value(4));
    println!("{}", c1.value(2));
    println!("{}", c1.value(5));


    let mut c2 = Cacher::new( | x: i32 | {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        match x {
            -31 => "minus thirtyone",
            40  => "fourty",
            -68 => "minus sixtyeight",
            _   => "I DON'T KNOW",
        }
    });

    println!("{}", c2.value(40));
    println!("{}", c2.value(-31));
    println!("{}", c2.value(40));
    println!("{}", c2.value(-68));
}