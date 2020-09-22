use crate::sharedstate::SharedState;
use futures::Future;
use std::{
  pin::Pin,
  sync::{
    Arc,
    Mutex,
  },
  task::{
    Context,
    Poll,
  },
  thread,
  time::Duration,
};

pub struct TimerFuture {
  shared_state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
  pub fn new(duration: Duration) -> Self {
    let shared_state = Arc::new(Mutex::new(SharedState {
      completed: false,
      waker: None,
    }));

    let thread_shared_state = shared_state.clone();

    thread::spawn(move || {
      thread::sleep(duration);

      let mut shared_state = thread_shared_state.lock().unwrap();

      shared_state.completed = true;

      if let Some(waker) = shared_state.waker.take() {
        waker.wake()
      }
    });

    TimerFuture {
      shared_state,
    }
  }
}

impl Future for TimerFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
    let mut shared_state = self.shared_state.lock().unwrap();

    if shared_state.completed {
      Poll::Ready(())
    } else {
      shared_state.waker = Some(ctx.waker().clone());
      Poll::Pending
    }
  }
}
