use alloc::{boxed::Box, rc::Rc};
use core::{cell::RefCell, future::Future, time::Duration};

use vexide::prelude::*;

use super::{Command, SharedController};

#[derive(Clone)]
struct IntakeCommand {
    io: Rc<RefCell<IntakeIO>>,
}

impl Command for IntakeCommand {
    async fn run(&self) {
        Intake::intake_run_impl(&self.io).await;
    }

    fn end(&self) {
        Intake::intake_end_impl(&self.io);
    }
}

impl Intake {
    async fn intake_run_impl(
        io: &Rc<RefCell<IntakeIO>>,
    ) {
        let mut io = io.borrow_mut();
        println!("Intake");
        io.intake_motor.set_voltage(-Motor::V5_MAX_VOLTAGE).ok();
    }

    fn intake_end_impl(
        io: &Rc<RefCell<IntakeIO>>,
    ) {
        let mut io = io.borrow_mut();
        println!("Stop Intake");
        io.intake_motor.brake(BrakeMode::Coast).ok();
    }
}

#[derive(Debug)]
pub struct Intake {
    io: Rc<RefCell<IntakeIO>>,
}

impl Intake {
    const DEADZONE: f64 = 0.05;
    const DRIFT_CORRECTION: f64 = 0.01;

    pub fn new(motor: Motor) -> Self {
        Self {
            io: IntakeIO::new(motor),
        }
    }

    pub fn intake(&self) -> impl Command {
        IntakeCommand {
            io: self.io.clone(),
        }
    }
}

#[derive(Debug)]
struct IntakeIO {
    intake_motor: Motor,
}

impl IntakeIO {
    pub fn new(mut motor: Motor) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { intake_motor: motor }))
    }
}
