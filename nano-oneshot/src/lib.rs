//! # nano-oneshot
//!
//! A simple one-shot channel.
//!
//! # Examples
//!
//! ```
//! let (s, r) = ::nano_oneshot::channel();
//!
//! let _ = s.send("hello");
//! assert_eq!(r.recv().unwrap(), "hello");
//! ```

use std::sync::Arc;
use std::time::Duration;

use parking_lot::{Condvar, Mutex, MutexGuard};

/// Creates a new one-shot channel.
///
/// Two halves are returned; a sender and a receiver. Each half is separately
/// owned and can be sent across threads.
///
/// # Examples
///
/// ```no_run
/// use std::thread;
/// use std::time::Duration;
///
/// let (s, r) = ::nano_oneshot::channel();
///
/// thread::spawn(move || {
///     thread::sleep(Duration::from_secs(1));
///     let _ = s.send(128);
/// });
///
/// assert_eq!(r.recv_timeout(Duration::from_secs(3)).unwrap(), 128);
/// ```
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let mutex = Arc::new(Mutex::new(None));
    let condvar = Arc::new(Condvar::new());

    let mutex2 = Arc::clone(&mutex);
    let condvar2 = Arc::clone(&condvar);

    let s = Sender(mutex, condvar);
    let r = Receiver(mutex2, condvar2);

    (s, r)
}

/// The sending half of a one-shot channel.
///
/// Senders are created by the [`channel`] function.
///
/// [`channel`]: fn.channel.html
#[derive(Debug)]
pub struct Sender<T>(Arc<Mutex<Option<T>>>, Arc<Condvar>);

impl<T> Sender<T> {
    /// Sends a value through the one-shot channel.
    ///
    /// If the receiving end of the channel has been dropped then an `Err` is
    /// returned with the value that was provided.
    pub fn send(self, value: T) -> Result<(), SendError<T>> {
        let Self(mutex, condvar) = &self;

        if self.is_disconnected() {
            return Err(SendError::Disconnected(value));
        }

        *mutex.lock() = Some(value);
        let _ = condvar.notify_one();

        Ok(())
    }

    /// Returns `true` if this `Sender` is disconnected.
    pub fn is_disconnected(&self) -> bool {
        let Self(mutex, condvar) = &self;

        Arc::strong_count(mutex) == 1 || Arc::strong_count(condvar) == 1
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let Self(_, condvar) = self;

        let _ = condvar.notify_one();
    }
}

/// The error returned by [`Sender::send`].
///
/// [`Sender::send`]: struct.Sender.html#method.send
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SendError<T> {
    Disconnected(T),
}

impl<T> SendError<T> {
    /// Consumes this error and unwraps the inner value.
    pub fn into_inner(self) -> T {
        match self {
            SendError::Disconnected(value) => value,
        }
    }
}

/// The receiving half of a one-shot channel.
///
/// Receivers are created by the [`channel`] function.
///
/// [`channel`]: fn.channel.html
pub struct Receiver<T>(Arc<Mutex<Option<T>>>, Arc<Condvar>);

impl<T> Receiver<T> {
    fn _recv<E, F>(self, disconnect_err: E, cond_fn: F) -> Result<T, E>
    where
        F: Fn(&mut MutexGuard<'_, Option<T>>, &Arc<Condvar>) -> Result<(), E>,
    {
        let Self(mut mutex, condvar) = self;

        loop {
            match Arc::try_unwrap(mutex) {
                Ok(mutex) => {
                    return mutex.into_inner().ok_or(disconnect_err);
                }
                Err(mutex_) => {
                    mutex = mutex_;
                    cond_fn(&mut mutex.lock(), &condvar)?;
                }
            }
        }
    }
}

impl<T> Receiver<T> {
    /// Blocks the current thread until a value is received or the channel is
    /// disconnected.
    pub fn recv(self) -> Result<T, RecvError> {
        self._recv(RecvError::Disconnected, |mut guard, condvar| {
            condvar.wait(&mut guard);
            Ok(())
        })
    }

    /// Blocks the current thread until a value is received, but only for a
    /// limited time.
    ///
    /// If the channel is disconnected before the timeout has elapsed then this
    /// method will wake up and return an `Err`.
    pub fn recv_timeout(self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        self._recv(RecvTimeoutError::Disconnected, move |mut guard, condvar| {
            if condvar.wait_for(&mut guard, timeout).timed_out() {
                Err(RecvTimeoutError::TimedOut)
            } else {
                Ok(())
            }
        })
    }

    /// Returns `true` if this receiver is disconnected.
    pub fn is_disconnected(&self) -> bool {
        let Self(mutex, condvar) = self;

        Arc::strong_count(mutex) == 1 || Arc::strong_count(condvar) == 1
    }
}

/// The error returned by [`Receiver::recv`].
///
/// [`Receiver::recv`]: struct.Receiver.html#method.recv
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RecvError {
    Disconnected,
}

/// The error returned by [`Receiver::recv_timeout`].
///
/// [`Receiver::recv_timeout`]: struct.Receiver.html#method.recv_timeout
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RecvTimeoutError {
    TimedOut,
    Disconnected,
}
