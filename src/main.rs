#![no_main]
#![no_std]
#![feature(async_closure)]
#![allow(clippy::await_holding_refcell_ref)]

extern crate alloc;

use core::time::Duration;

use log::info;
use subsystems::{
    drivetrain::{self, DifferentialDrivetrain},
    intake::{self, Intake, IntakeDirection},
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
        info!("Autonomous!");
    }

    async fn driver(&mut self) {
        info!("Driver!");

        loop {
            let state = self.controller.state().unwrap_or_default();

            self.drivetrain.set_goal(drivetrain::arcade(state));

            self.intake
                .while_pressed(state.right_trigger_1, intake::intake(IntakeDirection::Intake));
            self.intake
                .while_pressed(state.right_trigger_2, intake::intake(IntakeDirection::Outtake));

            sleep(Duration::from_millis(10)).await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    xyv::init_logger();

    let robot = Robot {
        controller: peripherals.primary_controller,
        drivetrain: DifferentialDrivetrain::new(
            Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
            Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
        ), //intake
        intake: Intake::new(Motor::new(
            peripherals.port_3,
            Gearset::Green,
            Direction::Reverse
        )),
        //intake extender
        //arm head
        //arm base
        //arm extender
        //climb
    };

    robot.compete().await;
}
