#[cfg(test)]
mod tests {
    use macos_security_logs::{
        collector::unified::UnifiedLogCollector,
        security::permissions::SecurityContext,
        models::log_entry::LogEntry,
    };
    use tokio;

    #[tokio::test]
    async fn test_security_context_creation() {
        let context = SecurityContext::new();
        assert!(context.is_ok());
    }

    #[tokio::test]
    async fn test_log_collector_creation() {
        let collector = UnifiedLogCollector::new(
            "subsystem == \"com.apple.security\"",
            100
        );
        
        assert_eq!(collector.max_entries, 100);
    }

    #[tokio::test]
    async fn test_permission_check() {
        let context = SecurityContext::new().unwrap();
        let has_permissions = context.has_sufficient_permissions();
        // Note: This test might fail if not run as root
        assert!(has_permissions.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        use std::time::Duration;
        use macos_security_logs::utils::rate_limiter::RateLimiter;

        let mut limiter = RateLimiter::new(
            Duration::from_millis(100),
            10,
            Duration::from_secs(60)
        );

        assert!(limiter.wait().await.is_ok());
    }
}