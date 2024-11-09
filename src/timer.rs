struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn reset(&mut self) {
        self.start = std::time::Instant::now();
    }

    pub fn elapsed(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
}

#[cfg(debug_assertions)]
pub const DEFAULT_COUNTDOWN_SECOND: f64 = 5.0;
#[cfg(not(debug_assertions))]
pub const DEFAULT_COUNTDOWN_SECOND: f64 = 1800.0;

pub struct CountDown {
    timer: Timer,
    time: f64,
    /// 标记 done_once
    onced: bool,
}

impl CountDown {
    pub fn new(time: f64) -> Self {
        Self {
            timer: Timer::default(),
            time,
            onced: false,
        }
    }
    pub fn reset(&mut self) {
        self.timer.reset();
        self.onced = false;
    }
    pub fn elapsed(&self) -> f64 {
        self.timer.elapsed()
    }
    pub fn done(&self) -> bool {
        self.elapsed() >= self.time
    }
    /// 只会在 done 后的第一次调用返回 true，否则返回 false
    pub fn done_once(&mut self) -> bool {
        if !self.onced && self.done() {
            self.onced = true;
            true
        } else {
            false
        }
    }
    pub fn time_left(&self) -> f64 {
        (self.time - self.elapsed()).max(0.0)
    }
}

impl Default for CountDown {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            time: DEFAULT_COUNTDOWN_SECOND,
            onced: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_done_once() {
        let mut count_down = CountDown::new(0.1);
        assert!(!count_down.done_once());
        std::thread::sleep(std::time::Duration::from_secs_f64(0.2));
        assert!(count_down.done_once());
        assert!(!count_down.done_once());
        assert!(!count_down.done_once());
    }
}
