use core::{cell::RefCell, future::Future, time::Duration};

use alloc::{boxed::Box, rc::Rc};
use vexide::{prelude::*};

use super::{SharedController, Subsystem};

trait Command {
    fn run(&self) -> impl Future<Output = ()>;
    fn end(&self);
}

#[derive(Clone)]
struct DriveWithController {
    controller: SharedController,
    io: Rc<RefCell<DifferentialDrivetrainIO>>,
}

impl Command for DriveWithController {
    async fn run(&self) {
        DifferentialDrivetrain::drive_with_controller_run_impl(&self.controller, &self.io).await;
    }

    fn end(&self) {
        
    }
}

impl DifferentialDrivetrain {
    async fn drive_with_controller_run_impl(controller: &SharedController, io: &Rc<RefCell<DifferentialDrivetrainIO>>) {
        let mut io = io.borrow_mut();
        loop {
            let state = controller.state();
            let forward = state.left_stick.y();
            let turn = state.right_stick.x();

            if forward >= Self::DEADZONE || turn >= Self::DEADZONE {
                if forward.abs() > 0.05 || turn.abs() > 0.05 {
                    let left_voltage = (turn + forward) * Motor::V5_MAX_VOLTAGE;
                    let right_voltage = (turn - forward) * Motor::V5_MAX_VOLTAGE;
        
                    // Set the drive motors to our arcade control values.
                    io.left.set_voltage(left_voltage).ok();
                    io.right.set_voltage(right_voltage).ok();
                } else {
                    io.left.brake(BrakeMode::Brake).ok();
                    io.right.brake(BrakeMode::Brake).ok();
                }
            }
            sleep(Controller::UPDATE_INTERVAL).await;
        }
    }
}

#[derive(Debug)]
pub struct DifferentialDrivetrain {
    io: Rc<RefCell<DifferentialDrivetrainIO>>,
}

impl DifferentialDrivetrain {
    const DEADZONE: f64 = 0.05;

    pub async fn drive_with_controller(self: Rc<Self>, controller: SharedController) -> impl Command {
        DriveWithController {
            controller,
            io: self.io.clone()
        }
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
