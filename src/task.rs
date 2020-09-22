use futures::Future;
use futures_task::ArcWake;
use std::{
  pin::Pin,
  sync::{
    mpsc::SyncSender,
    Arc,
    Mutex,
  },
};

/// A future that can reschedule itself to be polled by an executor.
pub struct Task {
  /// In-progress future that should be pushed to completion.
  pub future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,

  /// Handle for placing the task itself back onto the task queue.
  pub task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
  fn wake_by_ref(arc_self: &Arc<Self>) {
    let cloned = arc_self.clone();

    arc_self.task_sender.send(cloned).expect("Too many tasks queued");
  }
}
