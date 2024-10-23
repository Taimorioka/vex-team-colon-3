use alloc::rc::Rc;
use core::cell::RefCell;

use vexide::prelude::*;

use super::{goal, Goal, Subsystem};

type IntakeRef = Rc<RefCell<Intake>>;

pub fn idle() -> Goal<Intake> {
    goal(move |subsystem: IntakeRef| async move {
        let mut s = subsystem.borrow_mut();

        loop {
            _ = s.intake_motor.brake(BrakeMode::Coast);

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    })
}

pub fn intake(going_in: bool) -> Goal<Intake> {
    goal(move |subsystem: IntakeRef| async move {
        let mut s = subsystem.borrow_mut();

        loop {
            let volts = Motor::V5_MAX_VOLTAGE * if going_in { 1.0 } else { -1.0 };
            _ = s.intake_motor.set_voltage(volts);

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    })
}

#[derive(Debug)]
pub struct Intake {
    intake_motor: Motor,
}

impl Intake {
    pub fn new(intake_motor: Motor) -> Subsystem<Self> {
        Subsystem::new(Self { intake_motor }, idle())
    }
}
