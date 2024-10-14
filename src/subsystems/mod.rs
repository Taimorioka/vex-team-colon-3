use core::{cell::{Cell, RefCell, RefMut}, future::Future, ops::Deref};
use alloc::{rc::Rc, sync::Arc};
use snafu::Snafu;
use vexide::{devices::controller::ControllerState, prelude::*};

mod drivetrain;

#[derive(Debug)]
struct Subsystem<T> {
    io: Rc<RefCell<T>>,
    task: Option<Task<()>>,
}

impl<T> Subsystem<T> {
    fn start_command<F, R>(&mut self, command: F)
        where F: FnOnce(RefMut<T>) -> R,
        R: Future<Output = ()> + 'static
     {
        self.task = None;
        let io = self.io.clone();
        self.task = Some(spawn(command(io.borrow_mut())));
    }

    fn stop_command(&mut self) {
        self.task = None;
    }
}

type SharedController = Rc<RefreshableController>;

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
    }

    pub fn state(&self) -> ControllerState {
        *self.state.borrow()
    }
}