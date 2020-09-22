use constants::MAX_QUEUED_TASKS;
use executor::Executor;
use spawner::Spawner;
use std::{
  sync::mpsc::sync_channel,
  time::Duration,
};
use task::Task;
use timerfuture::TimerFuture;

mod constants;
mod executor;
mod sharedstate;
mod spawner;
mod task;
mod timerfuture;

/// Helper function.
///
/// Orchestrates communication between the `Executor` and the `Spawner`.
fn new_executor_and_spawner() -> (Executor, Spawner) {
  let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

  (
    Executor {
      ready_queue,
    },
    Spawner {
      task_sender,
    },
  )
}

fn main() {
  let (executor, spawner) = new_executor_and_spawner();

  // Spawn a task to print before and after waiting on a timer.
  spawner.spawn(async {
    println!("howdy!");

    // Wait for our timer future to complete after two seconds.
    TimerFuture::new(Duration::new(2, 0)).await;

    println!("done!");
  });

  // Drop the spawner so that our executor knows it is finished and won't receive more incoming tasks to run.
  drop(spawner);

  // Run the executor until the task queue is empty.
  //
  // This will print "howdy!", pause, and then print "done!".
  executor.run();
}
