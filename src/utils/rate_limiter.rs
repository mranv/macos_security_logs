use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct RateLimiter {
    last_request: Instant,
    min_delay: Duration,
}

impl RateLimiter {
    pub fn new(min_delay: Duration) -> Self {
        Self {
            last_request: Instant::now(),
            min_delay,
        }
    }

    pub async fn wait(&mut self) {
        let elapsed = self.last_request.elapsed();
        if elapsed < self.min_delay {
            sleep(self.min_delay - elapsed).await;
        }
        self.last_request = Instant::now();
    }
}
