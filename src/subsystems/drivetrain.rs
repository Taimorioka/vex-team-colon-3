use alloc::{boxed::Box, rc::Rc};
use core::{cell::RefCell, future::Future, time::Duration};

use vexide::prelude::*;

use super::{Command, SharedController};

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
        DifferentialDrivetrain::drive_with_controller_end_impl(&self.controller, &self.io);
    }
}

impl DifferentialDrivetrain {
    async fn drive_with_controller_run_impl(
        controller: &SharedController,
        io: &Rc<RefCell<DifferentialDrivetrainIO>>,
    ) {
        let mut io = io.borrow_mut();
        loop {
            let state = controller.state();
            let forward = -state.left_stick.y();
            let turn = state.right_stick.x() - Self::DRIFT_CORRECTION;

            if forward.abs() >= Self::DEADZONE || turn.abs() >= Self::DEADZONE {
                    let left_voltage = (turn + forward) * Motor::V5_MAX_VOLTAGE;
                    let right_voltage = (turn - forward) * Motor::V5_MAX_VOLTAGE;

                    // Set the drive motors to our arcade control values.
                    io.left.set_voltage(left_voltage).ok();
                    io.right.set_voltage(right_voltage).ok();
                } else {
                    io.left.brake(BrakeMode::Brake).ok();
                    io.right.brake(BrakeMode::Brake).ok();
                }
            sleep(Controller::UPDATE_INTERVAL).await;
        }
    }

    fn drive_with_controller_end_impl(
        controller: &SharedController,
        io: &Rc<RefCell<DifferentialDrivetrainIO>>,
    ) {
        let mut io = io.borrow_mut();
        _ = io;
    }
}

#[derive(Debug)]
pub struct DifferentialDrivetrain {
    io: Rc<RefCell<DifferentialDrivetrainIO>>,
}

impl DifferentialDrivetrain {
    const DEADZONE: f64 = 0.05;
    const DRIFT_CORRECTION: f64 = 0.01;

    pub fn new(left: Motor, right: Motor) -> Self {
        Self {
            io: DifferentialDrivetrainIO::new(left, right),
        }
    }

    pub fn drive_with_controller(&self, controller: SharedController) -> impl Command {
        DriveWithController {
            controller,
            io: self.io.clone(),
        }
    }
}

#[derive(Debug)]
struct DifferentialDrivetrainIO {
    left: Motor,
    right: Motor,
}

impl DifferentialDrivetrainIO {
    pub fn new(mut left: Motor, mut right: Motor) -> Rc<RefCell<Self>> {
        _ = left.brake(BrakeMode::Brake);
        _ = right.brake(BrakeMode::Brake);
        Rc::new(RefCell::new(Self { left, right }))
    }
}
