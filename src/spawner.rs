use crate::task::Task;
use futures::Future;
use std::sync::{
  mpsc::SyncSender,
  Arc,
  Mutex,
};

/// Spawns new futures onto the task channel.
#[derive(Clone)]
pub struct Spawner {
  pub task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
  pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
    let pinned_future = Box::pin(future);

    let task = Arc::new(Task {
      future: Mutex::new(Some(pinned_future)),
      task_sender: self.task_sender.clone(),
    });

    self.task_sender.send(task).expect("Too many tasks queued");
  }
}
