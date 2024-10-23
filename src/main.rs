#![no_main]
#![no_std]
#![feature(async_closure)]
#![allow(clippy::await_holding_refcell_ref)]

extern crate alloc;

use alloc::rc::Rc;
use core::time::Duration;

use subsystems::{
    drivetrain::{self, DifferentialDrivetrain},
    intake::{self, Intake},
    Subsystem,
};
use vexide::prelude::*;

mod subsystems;

struct Robot {
    controller: Controller,
    drivetrain: Subsystem<DifferentialDrivetrain>,
    intake: Subsystem<Intake>,
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");

        loop {
            let state = self.controller.state().unwrap_or_default();

            self.drivetrain.set_goal(drivetrain::arcade(state));

            self.intake
                .while_pressed(state.right_trigger_1, intake::intake(true));
            self.intake
                .while_pressed(state.right_trigger_2, intake::intake(false));

            sleep(Duration::from_millis(10)).await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {
        controller: peripherals.primary_controller,
        drivetrain: DifferentialDrivetrain::new(
            Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
            Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
        ), //intake
        intake: Intake::new(Motor::new(
            peripherals.port_3,
            Gearset::Green,
            Direction::Forward,
        )),
        //intake extender
        //arm head
        //arm base
        //arm extender
        //climb
    };

    robot.compete().await;
}
