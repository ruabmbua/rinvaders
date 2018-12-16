//! Utilities module (various stuff).

use js_sys::Math;

/// Generate a random number from 0 to (including) *max*. Uses Ecmascript Math.random() API under
/// the hood.
pub fn rand(max: u32) -> u32 {
    (Math::random() * (max + 1) as f64) as u32
}

/// Cap a number (or anything partially orderable) between *min* and *max*.
pub fn cap<N: PartialOrd>(val: N, min: N, max: N) -> N {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// A timer used for various game mechanics. It provies a repeated mode, and a one-shot mode.
/// The timer operates by receiving the game high res timestamp for every update.
pub struct Timer {
    // The future timestamp, on which a timer event should be raised.
    // --------------------------------------------------------------
    future_ts: u32,
    // Optional interval, with which the timer should be refreshed on timer event.
    // On event old `future_ts = real_event_time + interval`.
    // ---------------------------------------------------------------------------
    interval: Option<u32>,
}

impl Timer {
    /// Create a one shot timer, where *time* is the future timestamp on which to raise the timer
    /// event.
    pub fn once(time: u32) -> Self {
        Self {
            future_ts: time,
            interval: None,
        }
    }

    /// Create a timer interval. Takes *now* to calculate the first event. You can use now to specify
    /// a time offset from the real current timestamp, which can be used to start the timer interval
    /// at a later point in time. The *interval* specifies how long it takes the timer to fire a new
    /// event in repeated fashion.
    pub fn interval(now: u32, interval: u32) -> Self {
        Self {
            future_ts: now + interval,
            interval: Some(interval),
        }
    }

    /// Use this method in the game loop. Supply the current high res *ts*, and a closure to execute.
    /// The method will then only execute the callback closure, if a timer event was raised (e.g. ts reached, interval...).
    ///
    /// The closure is allowed to mutably capture references into its scope. It also receives an argument, which contains
    /// the time deviation in ms. Time deviation happens, if the game loop does not get enough updates. It will result in
    /// timer event being raised to late.
    /// You can use the deviation in the closure to determine how many times the closure content should be run (e.g. in
    /// a for loop), to correct the deviation in your game logic.
    ///
    /// The return value of this method is either `Some(result_of_callback)`, or None, when there was no event.
    ///
    /// TODO: Repair the interval refresh and callback calling logic, to include automatic handling of time deviation on low
    /// FPS, and refresh the timer correctly (Keep the interval correctly by using time deviation to calculate new *future_ts*).
    pub fn check<F, T>(&mut self, ts: u32, mut callback: F) -> Option<T>
    where
        F: FnMut(u32) -> T,
    {
        // When we are past (or at) the future ts, then raise an event.
        // ------------------------------------------------------------
        if ts >= self.future_ts {
            // Calculate the error.
            let off = ts - self.future_ts;

            // Call the callback with the time error.
            let ret = (callback)(off);

            // If this is a interval timer, refresh the value with the new one. The new value is not calculated based
            // on the old *future_ts*, but on the current ts (the time the last event was raised). This is probably
            // incorrect. TODO: Fix.
            // ------------------------------------------------------------------------------------------------------
            if let Some(i) = self.interval {
                self.future_ts = ts + i;
            }
            Some(ret)
        } else {
            None
        }
    }
}
