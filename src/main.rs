#![no_main]
#![no_std]

extern crate alloc;

use alloc::rc::Rc;
use core::time::Duration;

use subsystems::{
    drivetrain::DifferentialDrivetrain, intake::Intake, Command, RefreshableController, SharedController
};
use vexide::prelude::*;

mod subsystems;

struct Robot {
    controller: SharedController,
    drivetrain: DifferentialDrivetrain,
    intake: Intake,
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");

        let cmd = self
            .drivetrain
            .drive_with_controller(self.controller.clone())
            .schedule();

        let mut intake_cmd = None;

        loop {
            self.controller.refresh_or_default();
            let state = self.controller.state();

            if state.right_trigger_1.is_now_pressed() {
                intake_cmd = Some(self.intake.intake().schedule());
            }

            if state.right_trigger_1.is_now_released() {
                intake_cmd = None;
            }

            sleep(Duration::from_millis(10)).await;
        }
        drop(cmd);
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {
        controller: RefreshableController::shared(peripherals.primary_controller),
        drivetrain: DifferentialDrivetrain::new(
            Motor::new(peripherals.port_1, Gearset::Green, Direction::Forward),
            Motor::new(peripherals.port_2, Gearset::Green, Direction::Forward),
        ), //intake
        intake: Intake::new(Motor::new(peripherals.port_3, Gearset::Green, Direction::Forward)),
           //intake extender
           //arm head
           //arm base
           //arm extender
           //climb
    };

    robot.compete().await;
}
