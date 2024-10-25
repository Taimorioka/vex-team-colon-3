use alloc::rc::Rc;
use serde::Serialize;
use core::cell::RefCell;

use vexide::prelude::*;

use super::{goal, Goal, Subsystem};

type IntakeRef = Rc<RefCell<Intake>>;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum IntakeDirection {
    Intake,
    Outtake,
}

impl IntakeDirection {
    fn volts(&self) -> f64 {
        match self {
            Self::Intake => Motor::V5_MAX_VOLTAGE,
            Self::Outtake => -Motor::V5_MAX_VOLTAGE,
        }
    }
}

pub fn idle() -> Goal<Intake> {
    goal(move |subsystem: IntakeRef| async move {
        let mut s = subsystem.borrow_mut();
        xyv::record_output("/Intake/Direction", None::<IntakeDirection>);

        loop {
            _ = s.intake_motor.brake(BrakeMode::Coast);

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    })
}

pub fn intake(direction: IntakeDirection) -> Goal<Intake> {
    goal(move |subsystem: IntakeRef| async move {
        let mut s = subsystem.borrow_mut();

        xyv::record_output("/Intake/Direction", Some(direction));

        loop {
            _ = s.intake_motor.set_voltage(direction.volts());

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
