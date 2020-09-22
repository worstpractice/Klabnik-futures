use std::task::Waker;

pub struct SharedState {
  /// Whether or not the sleep time has elapsed.
  pub completed: bool,
  /// The "waker" to wake up the future.
  pub waker: Option<Waker>,
}
