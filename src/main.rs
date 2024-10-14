#![no_main]
#![no_std]

extern crate alloc;

use core::time::Duration;

use alloc::rc::Rc;
use subsystems::RefreshableController;
use vexide::prelude::*;

mod subsystems;

struct Robot {
    controller: Rc<RefreshableController>,
    motor_left: Motor,
    motor_right: Motor,
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
        
        loop {
            self.controller.refresh_or_default();
            let state = self.controller.state().unwrap_or_default();
            let forward = state.left_stick.y();
            let turn = state.right_stick.x();

            if forward.abs() > 0.05 || turn.abs() > 0.05 {
                let left_voltage = (turn + forward) * Motor::V5_MAX_VOLTAGE;
                let right_voltage = (turn - forward) * Motor::V5_MAX_VOLTAGE;
    
                // Set the drive motors to our arcade control values.
                self.motor_left.set_voltage(left_voltage).ok();
                self.motor_right.set_voltage(right_voltage).ok();
            } else {
                self.motor_left.brake(BrakeMode::Brake).ok();
                self.motor_right.brake(BrakeMode::Brake).ok();
            }


            sleep(Duration::from_millis(10)).await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {
        controller: RefreshableController::shared(peripherals.primary_controller),
        motor_left: Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
        motor_right: Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
        //intake
        //intake extender
        //arm head
        //arm base
        //arm extender
        //climb
    };
    
    robot.compete().await;
}
