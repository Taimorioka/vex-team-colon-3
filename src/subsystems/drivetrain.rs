use alloc::rc::Rc;
use core::cell::RefCell;

use vexide::{devices::controller::ControllerState, prelude::*};

use super::{goal, Goal, Subsystem};

const DEADZONE: f64 = 0.05;
const DRIFT_CORRECTION: f64 = 0.01;

type DrivetrainRef = Rc<RefCell<DifferentialDrivetrain>>;

pub fn idle() -> Goal<DifferentialDrivetrain> {
    goal(|subsystem: DrivetrainRef| async move {
        let mut s = subsystem.borrow_mut();

        loop {
            _ = s.left.brake(BrakeMode::Brake);
            _ = s.right.brake(BrakeMode::Brake);

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    })
}

pub fn arcade(state: ControllerState) -> Goal<DifferentialDrivetrain> {
    goal(move |subsystem: DrivetrainRef| async move {
        let mut s = subsystem.borrow_mut();

        loop {
            let forward = -state.left_stick.y();
            let turn = state.right_stick.x() - DRIFT_CORRECTION;

            if forward.abs() >= DEADZONE || turn.abs() >= DEADZONE {
                let left_voltage = (turn + forward) * Motor::V5_MAX_VOLTAGE;
                let right_voltage = (turn - forward) * Motor::V5_MAX_VOLTAGE;

                xyv::record_output("/Drivetrain/LeftVoltage", left_voltage);
                xyv::record_output("/Drivetrain/RightVoltage", right_voltage);

                // Set the drive motors to our arcade control values.
                s.left.set_voltage(left_voltage).ok();
                s.right.set_voltage(right_voltage).ok();
            } else {
                xyv::record_output("/Drivetrain/LeftVoltage", 0.0);
                xyv::record_output("/Drivetrain/RightVoltage", 0.0);

                s.left.brake(BrakeMode::Brake).ok();
                s.right.brake(BrakeMode::Brake).ok();
            }

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    })
}

#[derive(Debug)]
pub struct DifferentialDrivetrain {
    left: Motor,
    right: Motor,
}

impl DifferentialDrivetrain {
    pub fn new(left: Motor, right: Motor) -> Subsystem<Self> {
        Subsystem::new(Self { left, right }, idle())
    }
}
