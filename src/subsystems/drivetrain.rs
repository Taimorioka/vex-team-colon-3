use core::cell::RefCell;

use alloc::rc::Rc;
use vexide::{devices::controller::Joystick, prelude::*};

use super::Subsystem;

#[derive(Debug)]
pub struct DifferentialDrivetrain {
    subsystem: Subsystem<DifferentialDrivetrainIO>,
}

impl DifferentialDrivetrain {
    const DEADZONE: f64 = 0.05;

    pub async fn drive_with_controller(&mut self, arcade_drive: Joystick, arcade_turn: Joystick) -> Task<()> {
        spawn(async move {
            loop {
                let drive = arcade_drive.y().unwrap_or_default();
                let turn = arcade_turn.x().unwrap_or_default();
                let io = 
                if drive >= Self::DEADZONE || turn >= Self::DEADZONE {
                    let drive_volts = drive * Motor::MAX_VOLTAGE;
                    let turn_volts = turn * Motor::MAX_VOLTAGE;
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
