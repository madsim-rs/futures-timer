//! Support for creating futures that represent timeouts.
//!
//! This module contains the `Delay` type which is a future that will resolve
//! at a particular point in the future.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

/// A future representing the notification that an elapsed duration has
/// occurred.
///
/// This is created through the `Delay::new` method indicating when the future should fire.
/// Note that these futures are not intended for high resolution timers, but rather they will
/// likely fire some granularity after the exact instant that they're otherwise indicated to fire
/// at.
pub struct Delay(madsim::time::Sleep);

impl Delay {
    /// Creates a new future which will fire at `dur` time into the future.
    ///
    /// The returned object will be bound to the default timer for this thread.
    /// The default timer will be spun up in a helper thread on first use.
    #[inline]
    pub fn new(dur: Duration) -> Delay {
        Self(madsim::time::sleep(dur))
    }

    /// Resets this timeout to an new timeout which will fire at the time
    /// specified by `at`.
    #[inline]
    pub fn reset(&mut self, dur: Duration) {
        Pin::new(&mut self.0).reset(madsim::time::Instant::now() + dur);
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

impl fmt::Debug for Delay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Delay").finish()
    }
}
