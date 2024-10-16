use core::{cell::RefCell, future::Future};

use alloc::{boxed::Box, rc::Rc};
use vexide::{devices::controller::Joystick, prelude::*};

use super::Subsystem;

trait Command {
    fn run(&self) -> impl Future<Output = ()> + Send + Sync;
    fn end(&self);
}

#[derive(Debug)]
pub struct DifferentialDrivetrain {
    subsystem: Subsystem<DifferentialDrivetrainIO>,
}

impl DifferentialDrivetrain {
    const DEADZONE: f64 = 0.05;

    pub async fn drive_with_controller(&mut self, controller: Rc<>) -> Task<()> {
        spawn(async move {
            loop {
                let drive = arcade_drive.y().unwrap_or_default();
                let turn = arcade_turn.x().unwrap_or_default();
                if drive >= Self::DEADZONE || turn >= Self::DEADZONE {
                    let drive_volts = drive * Motor::V5_MAX_VOLTAGE;
                    let turn_volts = turn * Motor::V5_MAX_VOLTAGE;
                }
                sleep(Controller::UPDATE_INTERVAL).await;
            }
        })
    }
}

#[derive(Debug)]
struct DifferentialDrivetrainIO {
    left: Motor,
    right: Motor
}

impl DifferentialDrivetrainIO {
    pub fn new(mut left: Motor, mut right: Motor) -> Self {
        _ = left.brake(BrakeMode::Brake);
        _ = right.brake(BrakeMode::Brake);
        Self { left, right }
    }
}
