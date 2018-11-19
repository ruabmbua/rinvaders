use js_sys::Math;

pub fn rand(max: u32) -> u32 {
    (Math::random() * (max + 1) as f64) as u32
}

pub fn cap<N: PartialOrd>(val: N, min: N, max: N) -> N {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub struct Timer {
    future_ts: u32,
    interval: Option<u32>,
}

impl Timer {
    pub fn once(time: u32) -> Self {
        Self {
            future_ts: time,
            interval: None,
        }
    }

    pub fn interval(now: u32, interval: u32) -> Self {
        Self {
            future_ts: now + interval,
            interval: Some(interval),
        }
    }

    pub fn check<F, T>(&mut self, ts: u32, mut callback: F) -> Option<T>
    where
        F: FnMut(u32) -> T,
    {
        if ts >= self.future_ts {
            let off = ts - self.future_ts;
            let ret = (callback)(off);
            if let Some(i) = self.interval {
                self.future_ts = ts + i;
            }
            Some(ret)
        } else {
            None
        }
    }
}
