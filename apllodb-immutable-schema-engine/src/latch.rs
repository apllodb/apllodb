use std::{fmt::Debug, sync::Mutex};

/// Latch: Virtually same as "lock" but traditionally the term "lock" is used for transaction management context.
/// In contrast, "latch" is kind of mutex or semaphore to barrier data in implementation details.
#[derive(Debug)]
pub(crate) struct Latch<T: Debug>(Mutex<T>);

impl<T: Debug> Latch<T> {
    pub(crate) fn new(v: T) -> Self {
        Self(Mutex::new(v))
    }

    /// Mutex blocking RAII lock.
    pub(crate) fn with_lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut v = self
            .0
            .lock()
            .expect(format!("a thread panicked who took mutex lock to {:?}", self.0).as_str());
        f(&mut *v)
    }
}
