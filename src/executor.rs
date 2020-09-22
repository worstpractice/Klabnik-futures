use crate::Task;
use futures_task::waker_ref;
use std::{
  sync::{
    mpsc::Receiver,
    Arc,
  },
  task::{
    Context,
    Poll,
  },
};

/// Task executor that recieves tasks off of a channel and runs them.
pub struct Executor {
  pub ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
  pub fn run(&self) {
    while let Ok(task) = self.ready_queue.recv() {
      // Take the future, and if it has not yet completed (I.E: it is still `Some`), poll it in an attempt to complete it.
      let mut future_slot = task.future.lock().unwrap();

      if let Some(mut future) = future_slot.take() {
        // Create a `LocalWaker` from the `Task` itself.
        let waker = waker_ref(&task);

        let context = &mut Context::from_waker(&*waker);

        if let Poll::Pending = future.as_mut().poll(context) {
          // We're not done processing the future, so put it back in its task to be run again later.
          *future_slot = Some(future);
        }
      }
    }
  }
}
