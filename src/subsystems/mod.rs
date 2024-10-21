use alloc::{rc::Rc, sync::Arc};
use core::{
    cell::{Cell, RefCell, RefMut},
    future::Future,
    ops::Deref,
};

use snafu::Snafu;
use vexide::{devices::controller::ControllerState, prelude::*};

pub mod drivetrain;
pub mod intake;

pub type SharedController = Rc<RefreshableController>;

pub struct RefreshableController {
    state: RefCell<ControllerState>,
    inner: Controller,
}

impl RefreshableController {
    pub fn shared(inner: Controller) -> SharedController {
        Rc::new(Self {
            state: RefCell::new(inner.state().unwrap_or_default()),
            inner,
        })
    }

    pub fn refresh_or_default(&self) {
        *self.state.borrow_mut() = self.inner.state().unwrap_or_default();
        // println!("Controller: {:?}", self.state());
    }

    pub fn state(&self) -> ControllerState {
        *self.state.borrow()
    }
}

pub trait Command: 'static {
    fn run(&self) -> impl Future<Output = ()>
    where
        Self: Sized;
    fn end(&self);

    fn schedule(self) -> ScheduledCommand
    where
        Self: Sized,
    {
        ScheduledCommand::start(self)
    }
}

#[must_use = "the command is cancelled when the struct is dropped"]
pub struct ScheduledCommand {
    command: Rc<dyn Command>,
    task: Option<Task<()>>,
}

impl ScheduledCommand {
    pub fn start(command: impl Command) -> Self {
        println!("Start scheduled command");
        let command = Rc::new(command);
        Self {
            command: command.clone(),
            task: Some(spawn(async move {
                println!("Run scheduled command");
                command.run().await;
            })),
        }
    }

    pub fn stop(self) {}
}

impl Drop for ScheduledCommand {
    fn drop(&mut self) {
        println!("End scheduled command");
        if let Some(task) = self.task.take() {
            let command = self.command.clone();
            spawn(async move {
                task.cancel().await;
                command.end();
            }).detach();
        }
    }
}
