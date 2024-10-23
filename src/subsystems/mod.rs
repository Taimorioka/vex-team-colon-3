use alloc::{boxed::Box, rc::Rc};
use core::{cell::RefCell, future::Future, pin::Pin};

use vexide::{devices::controller::ButtonState, prelude::*};

pub mod drivetrain;
pub mod intake;

pub struct Subsystem<T> {
    data: Rc<RefCell<T>>,
    idle_goal: Goal<T>,
    current_goal: Option<Task<()>>,
}

impl<T> Subsystem<T> {
    pub fn new(data: T, idle_goal: Goal<T>) -> Self {
        let data = Rc::new(RefCell::new(data));
        Self {
            current_goal: Some(idle_goal.begin(data.clone())),
            data,
            idle_goal,
        }
    }

    pub fn set_goal(&mut self, goal: Goal<T>) {
        self.current_goal = None;
        self.current_goal = Some(goal.begin(self.data.clone()));
    }

    pub fn while_pressed(&mut self, button: ButtonState, goal: Goal<T>) {
        if button.is_now_pressed() {
            self.set_goal(goal);
        } else if button.is_now_released() {
            self.set_goal(self.idle_goal.clone());
        }
    }
}

pub struct Goal<T>(Rc<dyn Fn(Rc<RefCell<T>>) -> Pin<Box<dyn Future<Output = ()>>>>);

impl<T> Clone for Goal<T> {
    fn clone(&self) -> Self {
        Goal(self.0.clone())
    }
}

impl<T> Goal<T> {
    fn begin(&self, data: Rc<RefCell<T>>) -> Task<()> {
        spawn((self.0)(data))
    }
}

pub fn goal<T, G, F>(inner: G) -> Goal<T>
where
    G: Fn(Rc<RefCell<T>>) -> F + 'static,
    F: Future<Output = ()> + 'static,
{
    Goal(Rc::new(move |s| Box::pin(inner(s))))
}
