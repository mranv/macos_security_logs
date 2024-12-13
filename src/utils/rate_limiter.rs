use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct RateLimiter {
    last_request: Instant,
    min_delay: Duration,
    max_requests_per_interval: u32,
    request_count: u32,
    interval_start: Instant,
}

impl RateLimiter {
    pub fn new(min_delay: Duration, max_requests_per_interval: u32, interval: Duration) -> Self {
        Self {
            last_request: Instant::now(),
            min_delay,
            max_requests_per_interval,
            request_count: 0,
            interval_start: Instant::now(),
        }
    }

    pub async fn wait(&mut self) -> Result<(), String> {
        // Check if we need to reset the interval
        if self.interval_start.elapsed() >= Duration::from_secs(60) {
            self.request_count = 0;
            self.interval_start = Instant::now();
        }

        // Check rate limit
        if self.request_count >= self.max_requests_per_interval {
            return Err("Rate limit exceeded".to_string());
        }

        // Enforce minimum delay between requests
        let elapsed = self.last_request.elapsed();
        if elapsed < self.min_delay {
            sleep(self.min_delay - elapsed).await;
        }

        self.request_count += 1;
        self.last_request = Instant::now();
        Ok(())
    }
}