use std::time::{Duration, Instant};

struct Heartbeat {
    interval: Duration,
    last_sent: Instant,
}

impl Heartbeat {
    fn tick(&mut self) -> bool {
        if self.last_sent.elapsed() >= self.interval {
            self.last_sent = Instant::now();
            true
        } else {
            false
        }
    }
}
