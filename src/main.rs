#![no_main]
#![no_std]

use vexide::prelude::*;

struct Robot {}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    println!("Hello world");
    let robot = Robot {};

    robot.compete().await;
}