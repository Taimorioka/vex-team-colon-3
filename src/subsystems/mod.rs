use core::{cell::{RefCell, RefMut}, future::Future, ops::Deref};
use alloc::rc::Rc;
use snafu::Snafu;
use vexide::prelude::{spawn, Task};

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